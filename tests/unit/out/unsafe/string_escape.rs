extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const u16,
    pub __ctype_tolower: *const i32,
    pub __ctype_toupper: *const i32,
    pub __names: [*const libc::c_char; 13],
}
impl Clone for __locale_struct {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __locale_struct {
    fn default() -> Self {
        unimplemented!()
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut special: *const libc::c_char =
        c"\x07\x08\t\n\x0b\x0c\r !\"#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~\xff".as_ptr();
    let mut i: i32 = 0;
    'loop_: while ((i)
        < (((::std::mem::size_of::<[libc::c_char; 41]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize))) as i32))
    {
        (if (((*special.offset((i) as isize)) as i32)
            == ((*std::cell::LazyCell::force_mut(&mut *&raw mut expected_0))[(i) as usize] as i32))
        {
            &(0);
        } else {
            (unsafe {
                __assert_fail_1(
                    c"special[i] == expected[i]".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        i.postfix_inc();
    }
    return 0;
}
static mut expected_0: std::cell::LazyCell<[libc::c_char; 41]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            (7 as libc::c_char),
            (8 as libc::c_char),
            (9 as libc::c_char),
            (10 as libc::c_char),
            (11 as libc::c_char),
            (12 as libc::c_char),
            (13 as libc::c_char),
            (32 as libc::c_char),
            (33 as libc::c_char),
            (34 as libc::c_char),
            (35 as libc::c_char),
            (36 as libc::c_char),
            (37 as libc::c_char),
            (38 as libc::c_char),
            (39 as libc::c_char),
            (40 as libc::c_char),
            (41 as libc::c_char),
            (42 as libc::c_char),
            (43 as libc::c_char),
            (44 as libc::c_char),
            (45 as libc::c_char),
            (46 as libc::c_char),
            (47 as libc::c_char),
            (58 as libc::c_char),
            (59 as libc::c_char),
            (60 as libc::c_char),
            (61 as libc::c_char),
            (62 as libc::c_char),
            (63 as libc::c_char),
            (64 as libc::c_char),
            (91 as libc::c_char),
            (92 as libc::c_char),
            (93 as libc::c_char),
            (94 as libc::c_char),
            (95 as libc::c_char),
            (96 as libc::c_char),
            (123 as libc::c_char),
            (124 as libc::c_char),
            (125 as libc::c_char),
            (126 as libc::c_char),
            (b'\xff' as libc::c_char),
        ]
    });
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
pub unsafe fn __assert_fail_1(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
