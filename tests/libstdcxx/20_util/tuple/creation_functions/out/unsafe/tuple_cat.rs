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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut a3: () = unimplemented!();
    unimplemented!();
    let mut pdb: () = unimplemented!();
    let mut t: () = unimplemented!();
    let mut i: () = unimplemented!();
    let mut d: () = unimplemented!();
    let mut pi: *mut () = std::ptr::null_mut();
    let mut to: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(
        true,
        "std::is_same<decltype\n\t\t(std::tuple_cat(a3, pdb, t, a3, pdb, t)),\n                std::tuple<int, int, int, double, bool,\n\t\tunsigned, float, std::nullptr_t, void*,\n\t\tint, int, int, double, bool, unsigned,\n\t\tfloat, std::nullptr_t, void*>\n                >::value"
    );;
    unimplemented!();
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
