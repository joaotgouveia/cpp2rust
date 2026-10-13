extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn len_0(s: *const [libc::c_char; 5]) -> i32 {
    let mut n: i32 = 0;
    'loop_: while (((*s)[(n) as usize] as i32) != (('\0' as libc::c_char) as i32)) {
        n.prefix_inc();
    }
    return n;
}
pub unsafe fn len5_1(mut s: *const libc::c_char) -> i32 {
    let mut n: i32 = 0;
    'loop_: while (((*s.offset((n) as isize)) as i32) != (('\0' as libc::c_char) as i32)) {
        n.prefix_inc();
    }
    return n;
}
pub unsafe fn sum_2(a: *const [i32; 3]) -> i32 {
    return ((((*a)[(0) as usize]) + ((*a)[(1) as usize])) + ((*a)[(2) as usize]));
}
pub unsafe fn fill_3(a: *mut [i32; 3], mut v: i32) {
    let mut i: i32 = 0;
    'loop_: while ((i) < (3)) {
        (*a)[(i) as usize] = v;
        i.prefix_inc();
    }
}
pub unsafe fn sum_twice_4(a: *const [i32; 3]) -> i32 {
    return ((unsafe { sum_2(a) }) + (unsafe { sum_2(a) }));
}
pub unsafe fn sum_ptr_5(mut p: *const i32) -> i32 {
    return (((*p.offset((0) as isize)) + (*p.offset((1) as isize))) + (*p.offset((2) as isize)));
}
pub unsafe fn sum_decayed_6(a: *const [i32; 3]) -> i32 {
    return (unsafe { sum_ptr_5((*a).as_ptr()) });
}
pub unsafe fn bump_ptr_7(mut p: *mut i32) {
    (*p.offset((0) as isize)) += 1;
}
pub unsafe fn bump_decayed_8(a: *mut [i32; 3]) {
    (unsafe { bump_ptr_7((*a).as_mut_ptr()) });
}
pub unsafe fn fill_and_sum_9(a: *mut [i32; 3], mut v: i32, out: *mut i32) {
    (unsafe {
        let _a: *mut [i32; 3] = a;
        let _v: i32 = v;
        fill_3(_a, _v)
    });
    (*out) = (unsafe { sum_twice_4(a) });
}
pub unsafe fn pick_10(s: *const [libc::c_char; 5]) -> *const [libc::c_char; 5] {
    return s;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
pub unsafe fn sum_points_11(p: *const [Point; 2]) -> i32 {
    return (((((*p)[(0) as usize].x) + ((*p)[(0) as usize].y)) + ((*p)[(1) as usize].x))
        + ((*p)[(1) as usize].y));
}
pub unsafe fn shift_points_12(p: *mut [Point; 2], mut d: i32) {
    (*p)[(0) as usize].x += d;
    (*p)[(1) as usize].y += d;
}
pub unsafe fn total_len_13(names: *mut [*const libc::c_char; 2]) -> i32 {
    return ((unsafe { len5_1((*names)[(0) as usize]) })
        + (unsafe { len5_1((*names)[(1) as usize]) }));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if ((unsafe { len_0(&std::mem::transmute(*b"beta\0")) }) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"len(\"beta\") == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut buf: [libc::c_char; 5] = std::mem::transmute(*b"abcd\0");
    (if ((unsafe { len_0(&buf) }) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"len(buf) == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut arr: [i32; 3] = [1, 2, 3];
    (if ((unsafe { sum_2(&arr) }) == (6)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"sum(arr) == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe {
        let mut _a: [i32; 3] = [1, 2, 3];
        sum_2(&mut _a)
    }) == (6))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"sum({1, 2, 3}) == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { fill_3(&mut arr, 7) });
    (if ((unsafe { sum_2(&arr) }) == (21)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"sum(arr) == 21".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { sum_twice_4(&arr) }) == (42)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"sum_twice(arr) == 42".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut out: i32 = 0;
    (unsafe { fill_and_sum_9(&mut arr, 2, &mut out) });
    (if ((out) == (12)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"out == 12".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((arr[(0) as usize]) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"arr[0] == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let lit: *const [libc::c_char; 5] = &std::mem::transmute(*b"beta\0");
    (if ((unsafe { len_0(lit) }) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"len(lit) == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { pick_10(&std::mem::transmute(*b"beta\0")) }))[(0) as usize] as i32)
        == (('b' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"pick(\"beta\")[0] == \'b\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { len_0((unsafe { pick_10(&buf) })) }) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"len(pick(buf)) == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut pts: [Point; 2] = [Point { x: 1, y: 2 }, Point { x: 3, y: 4 }];
    (if ((unsafe { sum_points_11(&pts) }) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"sum_points(pts) == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { shift_points_12(&mut pts, 10) });
    (if ((pts[(0) as usize].x) == (11)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"pts[0].x == 11".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((pts[(1) as usize].y) == (14)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"pts[1].y == 14".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { sum_points_11(&pts) }) == (30)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"sum_points(pts) == 30".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { sum_decayed_6(&arr) }) == (unsafe { sum_2(&arr) })) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"sum_decayed(arr) == sum(arr)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { bump_decayed_8(&mut arr) });
    (if ((arr[(0) as usize]) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"arr[0] == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut names: [*const libc::c_char; 2] = [c"ab".as_ptr(), c"cde".as_ptr()];
    (if ((unsafe { total_len_13(&mut names) }) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"total_len(names) == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut rows: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    let mut p: *mut [i32; 3] = rows.as_mut_ptr();
    p.prefix_inc();
    (if (((*p).as_mut_ptr()) == (&mut rows[(1) as usize][(0) as usize] as *mut i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"*p == &rows[1][0]".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let r: *mut [i32; 3] = &mut (*p);
    (if (((*r)[(0) as usize]) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"r[0] == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { sum_2(r) }) == (15)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"sum(r) == 15".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { sum_2(&(*p)) }) == (15)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"sum(*p) == 15".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut q: *mut [i32; 3] = rows.as_mut_ptr();
    (if ((unsafe { sum_2(&(*q.offset((1) as isize))) }) == (15)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"sum(q[1]) == 15".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*q.offset((1) as isize)).as_mut_ptr())
        == (&mut rows[(1) as usize][(0) as usize] as *mut i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_14(
                c"q[1] == &rows[1][0]".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_14(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
