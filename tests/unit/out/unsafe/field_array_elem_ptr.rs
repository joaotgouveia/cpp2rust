extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Shape {
    pub id: i32,
    pub coords: [i32; 4],
    pub points: [Point; 3],
    pub tail: i32,
}
pub unsafe fn sum_0(mut p: *const i32, mut n: i32) -> i32 {
    let mut s: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        s += (*p.offset((i) as isize));
        i.prefix_inc();
    }
    return s;
}
pub unsafe fn set_y_1(mut p: *mut Point, mut y: i32) {
    (*p).y = y;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: Shape = Shape {
        id: 1,
        coords: [10, 20, 30, 40],
        points: [
            Point { x: 1, y: 2 },
            Point { x: 3, y: 4 },
            Point { x: 5, y: 6 },
        ],
        tail: 99,
    };
    let mut c: *mut i32 = (&mut s.coords[(1) as usize] as *mut i32);
    (if ((*c) == (20)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"*c == 20".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*c) = 21;
    (if ((s.coords[(1) as usize]) == (21)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"s.coords[1] == 21".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    c = (c).wrapping_add((2 as i32) as usize);
    (if ((*c) == (40)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"*c == 40".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((((c as usize - (&mut s.coords[(0) as usize] as *mut i32) as usize)
        / ::std::mem::size_of::<i32>()) as i64)
        == (3_i64))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"c - &s.coords[0] == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*c.offset((-1_i32) as isize)) == (30)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"c[-1] == 30".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { sum_0((s.coords.as_mut_ptr()).cast_const(), 4) }) == (101)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"sum(s.coords, 4) == 101".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { sum_0((&mut s.coords[(2) as usize] as *mut i32).cast_const(), 2) }) == (70)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"sum(&s.coords[2], 2) == 70".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut p: *mut Point = (&mut s.points[(1) as usize] as *mut Point);
    (if (((*p).x) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"p->x == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { set_y_1(p.offset((1) as isize), 60) });
    (if ((s.points[(2) as usize].y) == (60)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"s.points[2].y == 60".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut py: *mut i32 = (&mut s.points[(0) as usize].y as *mut i32);
    (*py) = 7;
    (if ((s.points[(0) as usize].y) == (7)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"s.points[0].y == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut px: *mut i32 = (&mut (*(p.offset((1) as isize))).x as *mut i32);
    (*px) += 50;
    (if ((s.points[(2) as usize].x) == (55)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"s.points[2].x == 55".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut sp: *mut Shape = (&mut s as *mut Shape);
    let mut d: *mut i32 = (*sp).coords.as_mut_ptr().offset((3) as isize);
    (*d) = 41;
    (if ((s.coords[(3) as usize]) == (41)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"s.coords[3] == 41".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*sp).points[(1) as usize].y = (((*sp).coords[(0) as usize]) + ((*sp).points[(0) as usize].x));
    (if ((s.points[(1) as usize].y) == (11)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"s.points[1].y == 11".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut q: *mut Point = (*sp).points.as_mut_ptr();
    (*q.offset((2) as isize)).x = 8;
    (if (((*sp).points[(2) as usize].x) == (8)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"sp->points[2].x == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut t: Shape = s;
    t.coords[(0) as usize] = 0;
    t.points[(0) as usize].x = 0;
    (if ((s.coords[(0) as usize]) == (10)) && ((s.points[(0) as usize].x) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"s.coords[0] == 10 && s.points[0].x == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((t.coords[(1) as usize]) == (21)) && ((t.points[(2) as usize].y) == (60)))
        && ((t.tail) == (99))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"t.coords[1] == 21 && t.points[2].y == 60 && t.tail == 99".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_2(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
