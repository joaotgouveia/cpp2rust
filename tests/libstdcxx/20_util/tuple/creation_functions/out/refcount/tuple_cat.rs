extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ tuple_cat_0() });
    ({
        let ___tpls: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        tuple_cat_1(___tpls.as_pointer())
    });
    ({
        let ___tpls_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ___tpls_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        tuple_cat_2(___tpls_0.as_pointer(), ___tpls_1.as_pointer())
    });
    let a3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ tuple_cat_3(a3.as_pointer()) });
    let pdb: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let pi: Value<Ptr<()>> = Rc::new(RefCell::new(Ptr::<()>::null()));
    let to: Value<()> = Rc::new(RefCell::new(
        std_tuple_int_ref__double_ref__int_ptrref_::new({ i.as_pointer() }, { d.as_pointer() }, {
            pi.as_pointer()
        }),
    ));
    ({ tuple_cat_4(pdb.as_pointer()) });
    ({ tuple_cat_5(to.as_pointer()) });
    ({
        let ___tpls_0: Ptr<()> = to.as_pointer();
        let ___tpls_1: Ptr<()> = to.as_pointer();
        tuple_cat_6(___tpls_0, ___tpls_1)
    });
    ({ tuple_cat_7(a3.as_pointer(), pdb.as_pointer()) });
    ({ tuple_cat_8(a3.as_pointer(), pdb.as_pointer(), t.as_pointer()) });
    ({
        let ___tpls_0: Ptr<()> = a3.as_pointer();
        let ___tpls_3: Ptr<()> = a3.as_pointer();
        tuple_cat_9(___tpls_0, pdb.as_pointer(), t.as_pointer(), ___tpls_3)
    });
    ({
        let ___tpls_0: Ptr<()> = a3.as_pointer();
        let ___tpls_1: Ptr<()> = pdb.as_pointer();
        let ___tpls_2: Ptr<()> = t.as_pointer();
        let ___tpls_3: Ptr<()> = a3.as_pointer();
        let ___tpls_4: Ptr<()> = pdb.as_pointer();
        let ___tpls_5: Ptr<()> = t.as_pointer();
        tuple_cat_10(
            ___tpls_0, ___tpls_1, ___tpls_2, ___tpls_3, ___tpls_4, ___tpls_5,
        )
    });
    const _: () = assert!(
        true,
        "std::is_same<decltype\n\t\t(std::tuple_cat(a3, pdb, t, a3, pdb, t)),\n                std::tuple<int, int, int, double, bool,\n\t\tunsigned, float, std::nullptr_t, void*,\n\t\tint, int, int, double, bool, unsigned,\n\t\tfloat, std::nullptr_t, void*>\n                >::value"
    );;
    ({
        let ___tpls_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ___tpls_1: Ptr<()> = to.as_pointer();
        let ___tpls_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ___tpls_4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ___tpls_6: Ptr<()> = to.as_pointer();
        tuple_cat_11(
            ___tpls_0.as_pointer(),
            ___tpls_1,
            a3.as_pointer(),
            ___tpls_3.as_pointer(),
            ___tpls_4.as_pointer(),
            pdb.as_pointer(),
            ___tpls_6,
        )
    });
    0
}
pub fn __cpp2rust_init_globals() {}
