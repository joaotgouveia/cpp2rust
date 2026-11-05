extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_0(s: Ptr<i8>) -> Ptr<i8> {
    let s: Value<Ptr<i8>> = Rc::new(RefCell::new(s));
    return (*s.borrow()).clone();
}
pub fn get_1(s: Ptr<i32>) -> Ptr<i32> {
    let s: Value<Ptr<i32>> = Rc::new(RefCell::new(s));
    return (*s.borrow()).clone();
}
pub fn get_2(s: Ptr<u8>) -> Ptr<u8> {
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(s));
    return (*s.borrow()).clone();
}
pub fn get_3(s: Ptr<u16>) -> Ptr<u16> {
    let s: Value<Ptr<u16>> = Rc::new(RefCell::new(s));
    return (*s.borrow()).clone();
}
pub fn get_4(s: Ptr<u32>) -> Ptr<u32> {
    let s: Value<Ptr<u32>> = Rc::new(RefCell::new(s));
    return (*s.borrow()).clone();
}
pub fn second_5(s: Ptr<i8>) -> i8 {
    return (elem!((s), 1).read());
}
pub fn second_6(s: Ptr<i32>) -> i32 {
    return (elem!((s), 1).read());
}
pub fn second_7(s: Ptr<u8>) -> u8 {
    return (elem!((s), 1).read());
}
pub fn second_8(s: Ptr<u16>) -> u16 {
    return (elem!((s), 1).read());
}
pub fn second_9(s: Ptr<u32>) -> u32 {
    return (elem!((s), 1).read());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut c: Ptr<i8> = Ptr::<i8>::from_string_literal(b"A");
    let mut w: Ptr<i32> = Ptr::<i32>::from_string_literal(&[65 as i32, 258 as i32, 0 as i32]);
    let mut b: Ptr<u8> = Ptr::<u8>::from_string_literal(&[196 as u8, 130 as u8, 0 as u8]);
    let mut s: Ptr<u16> = Ptr::<u16>::from_string_literal(&[65 as u16, 258 as u16, 0 as u16]);
    let mut l: Ptr<u32> = Ptr::<u32>::from_string_literal(&[65 as u32, 258 as u32, 0 as u32]);
    ({ cpp2rust_assert_fail_10((((elem!(c, 0).read()) as i32) == (('A' as i8) as i32))) });
    ({
        cpp2rust_assert_fail_10(
            (((elem!(b, 0).read()) as i32) == 196) && (((elem!(b, 1).read()) as i32) == 130),
        )
    });
    ({ cpp2rust_assert_fail_10(((elem!(w, 1).read()) == 258)) });
    ({ cpp2rust_assert_fail_10((((elem!(s, 1).read()) as i32) == 258)) });
    ({ cpp2rust_assert_fail_10(((elem!(l, 1).read()) == 258_u32)) });
    ({
        cpp2rust_assert_fail_10(
            (((elem!(w, 2).read()) == 0) && (((elem!(s, 2).read()) as i32) == 0))
                && ((elem!(l, 2).read()) == 0_u32),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            (((elem!(({ get_0(Ptr::<i8>::from_string_literal(b"A"),) }), 0).read()) as i32)
                == (('A' as i8) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            ((elem!(
                ({ get_1(Ptr::<i32>::from_string_literal(&[258 as i32, 0 as i32,]),) }),
                0
            )
            .read())
                == 258),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            (((elem!(
                ({
                    get_2(Ptr::<u8>::from_string_literal(&[
                        196 as u8, 130 as u8, 0 as u8,
                    ]))
                }),
                0
            )
            .read()) as i32)
                == 196),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            (((elem!(
                ({ get_3(Ptr::<u16>::from_string_literal(&[258 as u16, 0 as u16,]),) }),
                0
            )
            .read()) as i32)
                == 258),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            ((elem!(
                ({ get_4(Ptr::<u32>::from_string_literal(&[258 as u32, 0 as u32,]),) }),
                0
            )
            .read())
                == 258_u32),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            ((({ second_5(Ptr::<i8>::from_string_literal(b"AB")) }) as i32)
                == (('B' as i8) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            (({
                second_6(Ptr::<i32>::from_string_literal(&[
                    65 as i32, 258 as i32, 0 as i32,
                ]))
            }) == 258),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            ((({
                second_7(Ptr::<u8>::from_string_literal(&[
                    196 as u8, 130 as u8, 0 as u8,
                ]))
            }) as i32)
                == 130),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            ((({
                second_8(Ptr::<u16>::from_string_literal(&[
                    65 as u16, 258 as u16, 0 as u16,
                ]))
            }) as i32)
                == 258),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            (({
                second_9(Ptr::<u32>::from_string_literal(&[
                    65 as u32, 258 as u32, 0 as u32,
                ]))
            }) == 258_u32),
        )
    });
    let mut nw: u64 = (((::std::mem::size_of::<[i32; 4]>() as u64)
        .wrapping_div((::std::mem::size_of::<i32>() as u64)) as u64)
        .wrapping_sub(1_u64) as u64);
    let mut nb: u64 = (((::std::mem::size_of::<[u8; 4]>() as u64)
        .wrapping_div((::std::mem::size_of::<u8>() as u64)) as u64)
        .wrapping_sub(1_u64) as u64);
    let mut ns: u64 = (((::std::mem::size_of::<[u16; 4]>() as u64)
        .wrapping_div((::std::mem::size_of::<u16>() as u64)) as u64)
        .wrapping_sub(1_u64) as u64);
    let mut nl: u64 = (((::std::mem::size_of::<[u32; 4]>() as u64)
        .wrapping_div((::std::mem::size_of::<u32>() as u64)) as u64)
        .wrapping_sub(1_u64) as u64);
    ({
        cpp2rust_assert_fail_10(
            (((nw == 3_u64) && (nb == 3_u64)) && (ns == 3_u64)) && (nl == 3_u64),
        )
    });
    let mut pw: [i32; 4] = [258 as i32, 0 as i32, 0 as i32, 0 as i32];
    let mut ps: [u16; 4] = [258 as u16, 0 as u16, 0 as u16, 0 as u16];
    ({
        cpp2rust_assert_fail_10(
            ((pw[(0) as usize] == 258) && (pw[(1) as usize] == 0)) && (pw[(3) as usize] == 0),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            (((ps[(0) as usize] as i32) == 258) && ((ps[(1) as usize] as i32) == 0))
                && ((ps[(3) as usize] as i32) == 0),
        )
    });
    let mut ew: [i32; 2] = [0 as i32, 0 as i32];
    ({ cpp2rust_assert_fail_10((ew[(0) as usize] == 0) && (ew[(1) as usize] == 0)) });
    let mut wc: i32 = (258 as i32);
    let mut bc: u8 = (65 as u8);
    let mut sc: u16 = (258 as u16);
    let mut lc: u32 = (258 as u32);
    ({
        cpp2rust_assert_fail_10(
            (((wc == 258) && ((bc as i32) == 65)) && ((sc as i32) == 258)) && (lc == 258_u32),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            (((::std::mem::size_of::<i32>() == ::std::mem::size_of::<i32>())
                && (::std::mem::size_of::<u8>() == 1_u64))
                && (::std::mem::size_of::<u16>() == 2_u64))
                && (::std::mem::size_of::<u32>() == 4_u64),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            (((elem!(w, 1).read()) == (258 as i32))
                && (((elem!(s, 1).read()) as i32) == ((258 as u16) as i32)))
                && ((elem!(l, 1).read()) == (258 as u32)),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            (((elem!(b, 0).read()) as i32) == ((196 as u8) as i32))
                && (((elem!(b, 1).read()) as i32) == ((130 as u8) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            (({
                second_6(Ptr::<i32>::from_string_literal(&[
                    65 as i32, 258 as i32, 0 as i32,
                ]))
            }) == (258 as i32))
                && ((({
                    second_8(Ptr::<u16>::from_string_literal(&[
                        65 as u16, 258 as u16, 0 as u16,
                    ]))
                }) as i32)
                    == ((258 as u16) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            ((elem!(
                ({ get_4(Ptr::<u32>::from_string_literal(&[258 as u32, 0 as u32,]),) }),
                0
            )
            .read())
                == (258 as u32))
                && ((elem!(
                    ({ get_4(Ptr::<u32>::from_string_literal(&[258 as u32, 0 as u32,]),) }),
                    1
                )
                .read())
                    == (0 as u32)),
        )
    });
    ({
        cpp2rust_assert_fail_10(
            (((10 as i32) == 10) && (((9 as u16) as i32) == 9)) && ((92 as u32) == 92_u32),
        )
    });
    ({ cpp2rust_assert_fail_10((((97 as i32) + 1) == (98 as i32))) });
    ({ cpp2rust_assert_fail_10(((122 as u32).wrapping_sub((97 as u32)) == 25_u32)) });
    let mut wa: [i32; 3] = [(65 as i32), (258 as i32), (0 as i32)];
    ({
        cpp2rust_assert_fail_10(
            ((wa[(0) as usize] == 65) && (wa[(1) as usize] == 258)) && (wa[(2) as usize] == 0),
        )
    });
    wa[(0) as usize] = (66 as i32);
    ({ cpp2rust_assert_fail_10((wa[(0) as usize] == (('B' as i8) as i32))) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
