extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test02_0() {
    let mut ccarray1: Ptr<()> = Ptr::<()>::from_string_literal(&[
        115 as (), 97 as (), 110 as (), 32 as (), 102 as (), 114 as (), 97 as (), 110 as (),
        99 as (), 105 as (), 115 as (), 99 as (), 111 as (), 32 as (), 114 as (), 111 as (),
        111 as (), 102 as (), 32 as (), 103 as (), 97 as (), 114 as (), 100 as (), 101 as (),
        110 as (), 32 as (), 105 as (), 110 as (), 115 as (), 112 as (), 101 as (), 99 as (),
        116 as (), 111 as (), 114 as (), 115 as (), 0 as (),
    ]);
    let mut ccarray2: Ptr<()> = Ptr::<()>::from_string_literal(&[
        115 as (), 97 as (), 110 as (), 32 as (), 102 as (), 114 as (), 97 as (), 110 as (),
        99 as (), 105 as (), 115 as (), 99 as (), 111 as (), 32 as (), 115 as (), 117 as (),
        110 as (), 110 as (), 121 as (), 45 as (), 100 as (), 97 as (), 121 as (), 32 as (),
        112 as (), 97 as (), 114 as (), 107 as (), 32 as (), 105 as (), 110 as (), 115 as (),
        112 as (), 101 as (), 99 as (), 116 as (), 111 as (), 114 as (), 115 as (), 0 as (),
    ]);
    let carray: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..50).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    ({ wcscpy_1((carray.as_pointer() as Ptr<()>), (ccarray1).clone()) });
    let mut cw: Ptr<()> = Ptr::<()>::null();
    let mut w: Ptr<()> = Ptr::<()>::null();
    cw = ({
        let ___s: Ptr<()> = (ccarray1).clone();
        wmemchr_2(___s, (47 as ()), 3_())
    });
    cw = ({
        let ___wcs: Ptr<()> = (ccarray1).clone();
        wcschr_3(___wcs, (47 as ()))
    });
    cw = ({ wcspbrk_4((ccarray1).clone(), (ccarray2).clone()) });
    cw = ({
        let ___wcs: Ptr<()> = (ccarray1).clone();
        wcsrchr_5(___wcs, (99 as ()))
    });
    w = unimplemented!();
    cw = { (cw).clone() };
    w = { (w).clone() };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test02_0() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
