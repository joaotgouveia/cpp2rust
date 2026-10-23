extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
const _: () = assert!(
    true,
    "std::is_same<decltype(std::tuple_cat()),\n\t      std::tuple<>>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::tuple_cat\n\t\t\t\t    (std::declval<std::tuple<>>())),\n              std::tuple<>>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::tuple_cat\n\t\t\t\t    (std::declval<std::tuple<>&>())),\n              std::tuple<>>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::tuple_cat\n\t\t\t\t    (std::declval<const std::tuple<>>())),\n              std::tuple<>>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::tuple_cat\n\t\t\t\t    (std::declval<const std::tuple<>&>())),\n              std::tuple<>>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::tuple_cat\n\t\t\t\t    (std::declval<std::pair<int, bool>>())),\n              std::tuple<int, bool>>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::tuple_cat\n\t\t\t\t    (std::declval<std::pair<int, bool>&>())),\n              std::tuple<int, bool>>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype\n\t      (std::tuple_cat(std::declval<const std::pair<int, bool>>())),\n              std::tuple<int, bool>>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype\n\t      (std::tuple_cat(std::declval<const std::pair<int, bool>&>())),\n              std::tuple<int, bool>>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype\n\t      (std::tuple_cat(std::declval<std::array<int, 3>>())),\n              std::tuple<int, int, int>>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype\n\t      (std::tuple_cat(std::declval<std::array<int, 3>&>())),\n              std::tuple<int, int, int>>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype\n\t      (std::tuple_cat(std::declval<const std::array<int, 3>>())),\n              std::tuple<int, int, int>>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype\n\t      (std::tuple_cat(std::declval<const std::array<int, 3>&>())),\n              std::tuple<int, int, int>>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype\n\t      (std::tuple_cat\n\t       (std::declval<std::tuple<>>(), std::declval<std::tuple<>>())),\n              std::tuple<>>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype\n\t      (std::tuple_cat\n\t       (std::declval<std::tuple<>>(), std::declval<std::tuple<>>(),\n\t\tstd::declval<std::tuple<>>())), std::tuple<>>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype\n\t      (std::tuple_cat\n\t       (std::declval<std::tuple<>>(),\n\t\tstd::declval<std::array<char, 0>>(),\n\t\tstd::declval<std::array<int, 0>>(),\n\t\tstd::declval<std::tuple<>>())), std::tuple<>>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype\n\t      (std::tuple_cat\n\t       (std::declval<std::tuple<int>>(),\n\t\tstd::declval<std::tuple<double>>())),\n              std::tuple<int, double>>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype\n\t      (std::tuple_cat\n\t       (std::declval<std::tuple<int>>(),\n\t\tstd::declval<std::tuple<double>>(),\n\t\tstd::declval<std::tuple<const long&>>())),\n              std::tuple<int, double, const long&>>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype\n\t      (std::tuple_cat\n\t       (std::declval<std::array<wchar_t, 3>&>(),\n\t\tstd::declval<std::tuple<double>>(),\n\t\tstd::declval<std::tuple<>>(),\n\t\tstd::declval<std::tuple<unsigned&>>(),\n\t\tstd::declval<std::pair<bool, std::nullptr_t>>())),\n              std::tuple<wchar_t, wchar_t, wchar_t,\n\t      double, unsigned&, bool, std::nullptr_t>\n              >::value"
);
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { tuple_cat_0() });
    (unsafe {
        let mut ___tpls: () = unimplemented!();
        tuple_cat_1(&mut ___tpls)
    });
    (unsafe {
        let mut ___tpls_0: () = unimplemented!();
        let mut ___tpls_1: () = unimplemented!();
        tuple_cat_2(&mut ___tpls_0, &mut ___tpls_1)
    });
    let mut a3: () = unimplemented!();
    (unsafe { tuple_cat_3(&mut a3) });
    let mut pdb: () = unimplemented!();
    let mut t: () = unimplemented!();
    let mut i: () = unimplemented!();
    let mut d: () = unimplemented!();
    let mut pi: *mut () = std::ptr::null_mut();
    let mut to: () =
        std_tuple_int_ref__double_ref__int_ptrref_::new({ &mut i }, { &mut d }, { &mut pi });
    (unsafe { tuple_cat_4(&mut pdb) });
    (unsafe { tuple_cat_5(&mut to) });
    (unsafe {
        let ___tpls_0: *mut () = &mut to;
        let ___tpls_1: *mut () = &mut to;
        tuple_cat_6(___tpls_0, ___tpls_1)
    });
    (unsafe { tuple_cat_7(&mut a3, &mut pdb) });
    (unsafe { tuple_cat_8(&mut a3, &mut pdb, &mut t) });
    (unsafe {
        let ___tpls_0: *mut () = &mut a3;
        let ___tpls_3: *mut () = &mut a3;
        tuple_cat_9(___tpls_0, &mut pdb, &mut t, ___tpls_3)
    });
    (unsafe {
        let ___tpls_0: *mut () = &mut a3;
        let ___tpls_1: *mut () = &mut pdb;
        let ___tpls_2: *mut () = &mut t;
        let ___tpls_3: *mut () = &mut a3;
        let ___tpls_4: *mut () = &mut pdb;
        let ___tpls_5: *mut () = &mut t;
        tuple_cat_10(
            ___tpls_0, ___tpls_1, ___tpls_2, ___tpls_3, ___tpls_4, ___tpls_5,
        )
    });
    const _: () = assert!(
        true,
        "std::is_same<decltype\n\t\t(std::tuple_cat(a3, pdb, t, a3, pdb, t)),\n                std::tuple<int, int, int, double, bool,\n\t\tunsigned, float, std::nullptr_t, void*,\n\t\tint, int, int, double, bool, unsigned,\n\t\tfloat, std::nullptr_t, void*>\n                >::value"
    );;
    (unsafe {
        let mut ___tpls_0: () = unimplemented!();
        let ___tpls_1: *mut () = &mut to;
        let mut ___tpls_3: () = unimplemented!();
        let mut ___tpls_4: () = unimplemented!();
        let ___tpls_6: *mut () = &mut to;
        tuple_cat_11(
            &mut ___tpls_0,
            ___tpls_1,
            &mut a3,
            &mut ___tpls_3,
            &mut ___tpls_4,
            &mut pdb,
            ___tpls_6,
        )
    });
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
