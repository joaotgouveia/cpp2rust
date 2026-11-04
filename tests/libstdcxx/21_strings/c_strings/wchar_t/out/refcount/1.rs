extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let c: Value<i32> = Rc::new(RefCell::new((97 as i32)));
    let cc: Value<i32> = Rc::new(RefCell::new((98 as i32)));
    let mut c1: Ptr<i32> = (c.as_pointer());
    let mut cc1: Ptr<i32> = (cc.as_pointer());
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
    cc1 = ({
        let ___wcs: Ptr<i32> = (ccarray1).clone();
        wcschr_2(___wcs, (99 as i32))
    });
    c1 = ({ wcschr_3((carray.as_pointer() as Ptr<i32>), (99 as i32)) });
    cc1 = ({ wcspbrk_4((ccarray1).clone(), (ccarray2).clone()) });
    c1 = ({ wcspbrk_5((carray.as_pointer() as Ptr<i32>), (ccarray2).clone()) });
    cc1 = ({
        let ___wcs: Ptr<i32> = (ccarray1).clone();
        wcsrchr_6(___wcs, (99 as i32))
    });
    c1 = ({ wcsrchr_7((carray.as_pointer() as Ptr<i32>), (99 as i32)) });
    cc1 = ({ wcsstr_8((ccarray1).clone(), (ccarray2).clone()) });
    c1 = ({
        let ___s1: Ptr<i32> = (carray.as_pointer() as Ptr<i32>);
        let ___s2: Ptr<i32> = (carray.as_pointer() as Ptr<i32>);
        wcsstr_9(___s1, ___s2)
    });
    cc1 = ({
        let ___s: Ptr<i32> = (ccarray1).clone();
        wmemchr_10(___s, (97 as i32), 3_usize)
    });
    c1 = ({ wmemchr_11((carray.as_pointer() as Ptr<i32>), (97 as i32), 3_usize) });
    cc1 = { (cc1).clone() };
    c1 = { (c1).clone() };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
