extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct max_align_t {
    #[offset(0)]
    pub __clang_max_align_nonce1: i64,
    #[offset(16)]
    #[byte_size(16)]
    pub __clang_max_align_nonce2: f64,
}
impl Clone for max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_byte = u8;
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
    (if (((elem!(c, 0).read()) as i32) == (('A' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"c[0] == \'A\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((elem!(b, 0).read()) as i32) == 196) && (((elem!(b, 1).read()) as i32) == 130) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"b[0] == 0xC4 && b[1] == 0x82"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((elem!(w, 1).read()) == 258) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"w[1] == 0x102"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((elem!(s, 1).read()) as i32) == 258) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"s[1] == 0x102"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((elem!(l, 1).read()) == 258_u32) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"l[1] == 0x102"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((elem!(w, 2).read()) == 0) && (((elem!(s, 2).read()) as i32) == 0))
        && ((elem!(l, 2).read()) == 0_u32)
    {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"w[2] == 0 && s[2] == 0 && l[2] == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((elem!(({ get_0(Ptr::<i8>::from_string_literal(b"A"),) }), 0).read()) as i32)
        == (('A' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"get(\"A\")[0] == \'A\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((elem!(
        ({ get_1(Ptr::<i32>::from_string_literal(&[258 as i32, 0 as i32,]),) }),
        0
    )
    .read())
        == 258)
    {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"get(L\"\xc4\x82\")[0] == 0x102"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((elem!(
        ({
            get_2(Ptr::<u8>::from_string_literal(&[
                196 as u8, 130 as u8, 0 as u8,
            ]))
        }),
        0
    )
    .read()) as i32)
        == 196)
    {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"get(u8\"\xc4\x82\")[0] == 0xC4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((elem!(
        ({ get_3(Ptr::<u16>::from_string_literal(&[258 as u16, 0 as u16,]),) }),
        0
    )
    .read()) as i32)
        == 258)
    {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"get(u\"\xc4\x82\")[0] == 0x102"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((elem!(
        ({ get_4(Ptr::<u32>::from_string_literal(&[258 as u32, 0 as u32,]),) }),
        0
    )
    .read())
        == 258_u32)
    {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"get(U\"\xc4\x82\")[0] == 0x102"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ second_5(Ptr::<i8>::from_string_literal(b"AB")) }) as i32) == (('B' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"second(\"AB\") == \'B\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        second_6(Ptr::<i32>::from_string_literal(&[
            65 as i32, 258 as i32, 0 as i32,
        ]))
    }) == 258)
    {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"second(L\"A\xc4\x82\") == 0x102"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({
        second_7(Ptr::<u8>::from_string_literal(&[
            196 as u8, 130 as u8, 0 as u8,
        ]))
    }) as i32)
        == 130)
    {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"second(u8\"\xc4\x82\") == 0x82"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({
        second_8(Ptr::<u16>::from_string_literal(&[
            65 as u16, 258 as u16, 0 as u16,
        ]))
    }) as i32)
        == 258)
    {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"second(u\"A\xc4\x82\") == 0x102"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        second_9(Ptr::<u32>::from_string_literal(&[
            65 as u32, 258 as u32, 0 as u32,
        ]))
    }) == 258_u32)
    {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"second(U\"A\xc4\x82\") == 0x102"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut nw: usize =
        ((16usize as usize).wrapping_div((4usize as usize)) as usize).wrapping_sub(1_usize);
    let mut nb: usize =
        ((4usize as usize).wrapping_div((1usize as usize)) as usize).wrapping_sub(1_usize);
    let mut ns: usize =
        ((8usize as usize).wrapping_div((2usize as usize)) as usize).wrapping_sub(1_usize);
    let mut nl: usize =
        ((16usize as usize).wrapping_div((4usize as usize)) as usize).wrapping_sub(1_usize);
    (if (((nw == 3_usize) && (nb == 3_usize)) && (ns == 3_usize)) && (nl == 3_usize) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"nw == 3 && nb == 3 && ns == 3 && nl == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut pw: [i32; 4] = [258 as i32, 0 as i32, 0 as i32, 0 as i32];
    let mut ps: [u16; 4] = [258 as u16, 0 as u16, 0 as u16, 0 as u16];
    (if ((pw[(0) as usize] == 258) && (pw[(1) as usize] == 0)) && (pw[(3) as usize] == 0) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"pw[0] == 0x102 && pw[1] == 0 && pw[3] == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((ps[(0) as usize] as i32) == 258) && ((ps[(1) as usize] as i32) == 0))
        && ((ps[(3) as usize] as i32) == 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"ps[0] == 0x102 && ps[1] == 0 && ps[3] == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut ew: [i32; 2] = [0 as i32, 0 as i32];
    (if (ew[(0) as usize] == 0) && (ew[(1) as usize] == 0) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"ew[0] == 0 && ew[1] == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut wc: i32 = (258 as i32);
    let mut bc: u8 = (65 as u8);
    let mut sc: u16 = (258 as u16);
    let mut lc: u32 = (258 as u32);
    (if (((wc == 258) && ((bc as i32) == 65)) && ((sc as i32) == 258)) && (lc == 258_u32) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(
                    b"wc == 0x102 && bc == 0x41 && sc == 0x102 && lc == 0x102",
                ),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((4usize == 4usize) && (1usize == 1_usize)) && (2usize == 2_usize)) && (4usize == 4_usize)
    {
        &(0);
    } else {
        ({
            __assert_fail_10 ( Ptr::<i8 >::from_string_literal(b"sizeof(L\'a\') == sizeof(wchar_t) && sizeof(u8\'a\') == 1 && sizeof(u\'a\') == 2 && sizeof(U\'a\') == 4" )  , Ptr::<i8 >::from_string_literal(file!().as_bytes())  , line!() as u32    , Ptr::<i8 >::from_string_literal(b"int main()" )  , )
        });
    });
    (if (((elem!(w, 1).read()) == (258 as i32))
        && (((elem!(s, 1).read()) as i32) == ((258 as u16) as i32)))
        && ((elem!(l, 1).read()) == (258 as u32))
    {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(
                    b"w[1] == L\'\xc4\x82\' && s[1] == u\'\xc4\x82\' && l[1] == U\'\xc4\x82\'",
                ),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((elem!(b, 0).read()) as i32) == ((196 as u8) as i32))
        && (((elem!(b, 1).read()) as i32) == ((130 as u8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"b[0] == u8\'\\xC4\' && b[1] == u8\'\\x82\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        second_6(Ptr::<i32>::from_string_literal(&[
            65 as i32, 258 as i32, 0 as i32,
        ]))
    }) == (258 as i32))
        && ((({
            second_8(Ptr::<u16>::from_string_literal(&[
                65 as u16, 258 as u16, 0 as u16,
            ]))
        }) as i32)
            == ((258 as u16) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_10 ( Ptr::<i8 >::from_string_literal(b"second(L\"A\xc4\x82\") == L\'\xc4\x82\' && second(u\"A\xc4\x82\") == u\'\xc4\x82\'" )  , Ptr::<i8 >::from_string_literal(file!().as_bytes())  , line!() as u32    , Ptr::<i8 >::from_string_literal(b"int main()" )  , )
        });
    });
    (if ((elem!(
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
            == (0 as u32))
    {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(
                    b"get(U\"\xc4\x82\")[0] == U\'\xc4\x82\' && get(U\"\xc4\x82\")[1] == U\'\\0\'",
                ),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((10 as i32) == 10) && (((9 as u16) as i32) == 9)) && ((92 as u32) == 92_u32) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(
                    b"L\'\\n\' == 10 && u\'\\t\' == 9 && U\'\\\\\' == 92",
                ),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((97 as i32) + 1) == (98 as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"L\'a\' + 1 == L\'b\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((122 as u32).wrapping_sub((97 as u32)) == 25_u32) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"U\'z\' - U\'a\' == 25"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut wa: [i32; 3] = [(65 as i32), (258 as i32), (0 as i32)];
    (if ((wa[(0) as usize] == 65) && (wa[(1) as usize] == 258)) && (wa[(2) as usize] == 0) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"wa[0] == 0x41 && wa[1] == 0x102 && wa[2] == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    wa[(0) as usize] = (66 as i32);
    (if (wa[(0) as usize] == (('B' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"wa[0] == \'B\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_10(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
