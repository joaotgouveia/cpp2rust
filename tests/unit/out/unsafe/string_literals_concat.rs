extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut joined: *const libc::c_char = c"alpha\nbeta\ngamma\n".as_ptr();
    (if (((*joined.offset((0) as isize)) as i32) == (('a' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"joined[0] == \'a\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*joined.offset((5) as isize)) as i32) == (('\n' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"joined[5] == \'\\n\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*joined.offset((6) as isize)) as i32) == (('b' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"joined[6] == \'b\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut arr: [libc::c_char; 7] = std::mem::transmute(*b"foobar\0");
    (if ((arr[(0) as usize] as i32) == (('f' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr[0] == \'f\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((arr[(3) as usize] as i32) == (('b' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr[3] == \'b\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((arr[(5) as usize] as i32) == (('r' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr[5] == \'r\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((arr[(6) as usize] as i32) == (('\0' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr[6] == \'\\0\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut split_pieces: *const libc::c_char = c"abcdefghi".as_ptr();
    (if (((*split_pieces.offset((0) as isize)) as i32) == (('a' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"split_pieces[0] == \'a\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*split_pieces.offset((3) as isize)) as i32) == (('d' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"split_pieces[3] == \'d\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*split_pieces.offset((6) as isize)) as i32) == (('g' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"split_pieces[6] == \'g\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*split_pieces.offset((8) as isize)) as i32) == (('i' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"split_pieces[8] == \'i\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*split_pieces.offset((9) as isize)) as i32) == (('\0' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"split_pieces[9] == \'\\0\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_0(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
