extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: i64,
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
pub unsafe fn get_0(mut s: *const libc::c_char) -> *const libc::c_char {
    return s;
}
pub unsafe fn get_1(mut s: *const i32) -> *const i32 {
    return s;
}
pub unsafe fn get_2(mut s: *const u8) -> *const u8 {
    return s;
}
pub unsafe fn get_3(mut s: *const u16) -> *const u16 {
    return s;
}
pub unsafe fn get_4(mut s: *const u32) -> *const u32 {
    return s;
}
pub unsafe fn second_5(s: *const [libc::c_char; 3]) -> libc::c_char {
    return (*s)[(1) as usize];
}
pub unsafe fn second_6(s: *const [i32; 3]) -> i32 {
    return (*s)[(1) as usize];
}
pub unsafe fn second_7(s: *const [u8; 3]) -> u8 {
    return (*s)[(1) as usize];
}
pub unsafe fn second_8(s: *const [u16; 3]) -> u16 {
    return (*s)[(1) as usize];
}
pub unsafe fn second_9(s: *const [u32; 3]) -> u32 {
    return (*s)[(1) as usize];
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: *const libc::c_char = c"A".as_ptr();
    let mut w: *const i32 = [65 as i32, 258 as i32, 0 as i32].as_ptr();
    let mut b: *const u8 = [196 as u8, 130 as u8, 0 as u8].as_ptr();
    let mut s: *const u16 = [65 as u16, 258 as u16, 0 as u16].as_ptr();
    let mut l: *const u32 = [65 as u32, 258 as u32, 0 as u32].as_ptr();
    (if (((*c.offset((0) as isize)) as i32) == (('A' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"c[0] == \'A\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*b.offset((0) as isize)) as i32) == (196))
        && (((*b.offset((1) as isize)) as i32) == (130))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"b[0] == 0xC4 && b[1] == 0x82".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*w.offset((1) as isize)) == (258)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"w[1] == 0x102".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*s.offset((1) as isize)) as i32) == (258)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"s[1] == 0x102".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*l.offset((1) as isize)) == (258_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"l[1] == 0x102".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*w.offset((2) as isize)) == (0)) && (((*s.offset((2) as isize)) as i32) == (0)))
        && ((*l.offset((2) as isize)) == (0_u32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"w[2] == 0 && s[2] == 0 && l[2] == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { get_0(c"A".as_ptr()) }).offset((0) as isize)) as i32)
        == (('A' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"get(\"A\")[0] == \'A\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*(unsafe { get_1([258 as i32, 0 as i32].as_ptr()) }).offset((0) as isize)) == (258)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"get(L\"\xc4\x82\")[0] == 0x102".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { get_2([196 as u8, 130 as u8, 0 as u8].as_ptr()) }).offset((0) as isize))
        as i32)
        == (196))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"get(u8\"\xc4\x82\")[0] == 0xC4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { get_3([258 as u16, 0 as u16].as_ptr()) }).offset((0) as isize)) as i32)
        == (258))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"get(u\"\xc4\x82\")[0] == 0x102".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*(unsafe { get_4([258 as u32, 0 as u32].as_ptr()) }).offset((0) as isize)) == (258_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"get(U\"\xc4\x82\")[0] == 0x102".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { second_5(&std::mem::transmute(*b"AB\0")) }) as i32)
        == (('B' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"second(\"AB\") == \'B\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { second_6(&[65 as i32, 258 as i32, 0 as i32]) }) == (258)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"second(L\"A\xc4\x82\") == 0x102".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { second_7(&[196 as u8, 130 as u8, 0 as u8]) }) as i32) == (130)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"second(u8\"\xc4\x82\") == 0x82".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { second_8(&[65 as u16, 258 as u16, 0 as u16]) }) as i32) == (258)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"second(u\"A\xc4\x82\") == 0x102".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { second_9(&[65 as u32, 258 as u32, 0 as u32]) }) == (258_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"second(U\"A\xc4\x82\") == 0x102".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let nw: usize = ((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize)) as usize)
        .wrapping_sub(1_usize);
    let nb: usize = ((::std::mem::size_of::<[u8; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<u8>() as usize)) as usize)
        .wrapping_sub(1_usize);
    let ns: usize = ((::std::mem::size_of::<[u16; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<u16>() as usize)) as usize)
        .wrapping_sub(1_usize);
    let nl: usize = ((::std::mem::size_of::<[u32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<u32>() as usize)) as usize)
        .wrapping_sub(1_usize);
    (if ((((nw) == (3_usize)) && ((nb) == (3_usize))) && ((ns) == (3_usize))) && ((nl) == (3_usize))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"nw == 3 && nb == 3 && ns == 3 && nl == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let pw: [i32; 4] = [258 as i32, 0 as i32, 0 as i32, 0 as i32];
    let ps: [u16; 4] = [258 as u16, 0 as u16, 0 as u16, 0 as u16];
    (if (((pw[(0) as usize]) == (258)) && ((pw[(1) as usize]) == (0)))
        && ((pw[(3) as usize]) == (0))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"pw[0] == 0x102 && pw[1] == 0 && pw[3] == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((ps[(0) as usize] as i32) == (258)) && ((ps[(1) as usize] as i32) == (0)))
        && ((ps[(3) as usize] as i32) == (0))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"ps[0] == 0x102 && ps[1] == 0 && ps[3] == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let ew: [i32; 2] = [0 as i32, 0 as i32];
    (if ((ew[(0) as usize]) == (0)) && ((ew[(1) as usize]) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"ew[0] == 0 && ew[1] == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut wc: i32 = (258 as i32);
    let mut bc: u8 = (65 as u8);
    let mut sc: u16 = (258 as u16);
    let mut lc: u32 = (258 as u32);
    (if ((((wc) == (258)) && ((bc as i32) == (65))) && ((sc as i32) == (258)))
        && ((lc) == (258_u32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"wc == 0x102 && bc == 0x41 && sc == 0x102 && lc == 0x102".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((((::std::mem::size_of::<i32>()) == (::std::mem::size_of::<i32>()))
        && ((::std::mem::size_of::<u8>()) == (1_usize)))
        && ((::std::mem::size_of::<u16>()) == (2_usize)))
        && ((::std::mem::size_of::<u32>()) == (4_usize))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10 ( c"sizeof(L\'a\') == sizeof(wchar_t) && sizeof(u8\'a\') == 1 && sizeof(u\'a\') == 2 && sizeof(U\'a\') == 4" .as_ptr() , concat!(file!(), "\0").as_ptr() as *const libc::c_char  , line!() as u32  , c"int main()" .as_ptr() , )
        });
    });
    (if (((*w.offset((1) as isize)) == (258 as i32))
        && (((*s.offset((1) as isize)) as i32) == ((258 as u16) as i32)))
        && ((*l.offset((1) as isize)) == (258 as u32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"w[1] == L\'\xc4\x82\' && s[1] == u\'\xc4\x82\' && l[1] == U\'\xc4\x82\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*b.offset((0) as isize)) as i32) == ((196 as u8) as i32))
        && (((*b.offset((1) as isize)) as i32) == ((130 as u8) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"b[0] == u8\'\\xC4\' && b[1] == u8\'\\x82\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { second_6(&[65 as i32, 258 as i32, 0 as i32]) }) == (258 as i32))
        && (((unsafe { second_8(&[65 as u16, 258 as u16, 0 as u16]) }) as i32)
            == ((258 as u16) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10 ( c"second(L\"A\xc4\x82\") == L\'\xc4\x82\' && second(u\"A\xc4\x82\") == u\'\xc4\x82\'" .as_ptr() , concat!(file!(), "\0").as_ptr() as *const libc::c_char  , line!() as u32  , c"int main()" .as_ptr() , )
        });
    });
    (if ((*(unsafe { get_4([258 as u32, 0 as u32].as_ptr()) }).offset((0) as isize))
        == (258 as u32))
        && ((*(unsafe { get_4([258 as u32, 0 as u32].as_ptr()) }).offset((1) as isize))
            == (0 as u32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"get(U\"\xc4\x82\")[0] == U\'\xc4\x82\' && get(U\"\xc4\x82\")[1] == U\'\\0\'"
                    .as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((10 as i32) == (10)) && (((9 as u16) as i32) == (9))) && ((92 as u32) == (92_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"L\'\\n\' == 10 && u\'\\t\' == 9 && U\'\\\\\' == 92".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((97 as i32) + (1)) == (98 as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"L\'a\' + 1 == L\'b\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((122 as u32).wrapping_sub((97 as u32))) == (25_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"U\'z\' - U\'a\' == 25".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut wa: [i32; 3] = [(65 as i32), (258 as i32), (0 as i32)];
    (if (((wa[(0) as usize]) == (65)) && ((wa[(1) as usize]) == (258)))
        && ((wa[(2) as usize]) == (0))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"wa[0] == 0x41 && wa[1] == 0x102 && wa[2] == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    wa[(0) as usize] = (66 as i32);
    (if ((wa[(0) as usize]) == (('B' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_10(
                c"wa[0] == \'B\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_10(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
