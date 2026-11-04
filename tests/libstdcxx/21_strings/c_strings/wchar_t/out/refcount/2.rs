extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test02_0() {
    let mut ccarray1: Ptr<i32> = Ptr::<i32>::from_string_literal(&[
        115 as i32, 97 as i32, 110 as i32, 32 as i32, 102 as i32, 114 as i32, 97 as i32,
        110 as i32, 99 as i32, 105 as i32, 115 as i32, 99 as i32, 111 as i32, 32 as i32,
        114 as i32, 111 as i32, 111 as i32, 102 as i32, 32 as i32, 103 as i32, 97 as i32,
        114 as i32, 100 as i32, 101 as i32, 110 as i32, 32 as i32, 105 as i32, 110 as i32,
        115 as i32, 112 as i32, 101 as i32, 99 as i32, 116 as i32, 111 as i32, 114 as i32,
        115 as i32, 0 as i32,
    ]);
    let mut ccarray2: Ptr<i32> = Ptr::<i32>::from_string_literal(&[
        115 as i32, 97 as i32, 110 as i32, 32 as i32, 102 as i32, 114 as i32, 97 as i32,
        110 as i32, 99 as i32, 105 as i32, 115 as i32, 99 as i32, 111 as i32, 32 as i32,
        115 as i32, 117 as i32, 110 as i32, 110 as i32, 121 as i32, 45 as i32, 100 as i32,
        97 as i32, 121 as i32, 32 as i32, 112 as i32, 97 as i32, 114 as i32, 107 as i32, 32 as i32,
        105 as i32, 110 as i32, 115 as i32, 112 as i32, 101 as i32, 99 as i32, 116 as i32,
        111 as i32, 114 as i32, 115 as i32, 0 as i32,
    ]);
    let carray: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..50).map(|_| 0_i32).collect::<Box<[i32]>>()));
    ({ wcscpy_1((carray.as_pointer() as Ptr<i32>), (ccarray1).clone()) });
    let mut cw: Ptr<i32> = Ptr::<i32>::null();
    let mut w: Ptr<i32> = Ptr::<i32>::null();
    cw = ({
        let ___s: Ptr<i32> = (ccarray1).clone();
        wmemchr_2(___s, (47 as i32), 3_usize)
    });
    cw = ({
        let ___wcs: Ptr<i32> = (ccarray1).clone();
        wcschr_3(___wcs, (47 as i32))
    });
    cw = ({ wcspbrk_4((ccarray1).clone(), (ccarray2).clone()) });
    cw = ({
        let ___wcs: Ptr<i32> = (ccarray1).clone();
        wcsrchr_5(___wcs, (99 as i32))
    });
    w = ({
        let ___s1: Ptr<i32> = (carray.as_pointer() as Ptr<i32>);
        let ___s2: Ptr<i32> = (carray.as_pointer() as Ptr<i32>);
        wcsstr_6(___s1, ___s2)
    });
    cw = { (cw).clone() };
    w = { (w).clone() };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test02_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
