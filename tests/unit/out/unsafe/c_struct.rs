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
pub struct Line {
    pub start: Point,
    pub end: Point,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Node {
    pub value: i32,
    pub next: *mut Node,
}
pub type Color = u32;
pub const Color_RED: Color = 0;
pub const Color_GREEN: Color = 1;
pub const Color_BLUE: Color = 2;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Inner {
    pub a: i32,
    pub b: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Container {
    pub inner: Inner,
    pub color: Color,
    pub count: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: Point = Point { x: 10, y: 20 };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((p.x) == (10)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"p.x == 10".as_ptr().cast_mut()).cast_const(),
                        (c"c_struct.c".as_ptr().cast_mut()).cast_const(),
                        29_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((p.y) == (20)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"p.y == 20".as_ptr().cast_mut()).cast_const(),
                        (c"c_struct.c".as_ptr().cast_mut()).cast_const(),
                        30_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut q: Point = p;
    q.x = 99;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((p.x) == (10)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"p.x == 10".as_ptr().cast_mut()).cast_const(),
                        (c"c_struct.c".as_ptr().cast_mut()).cast_const(),
                        34_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((q.x) == (99)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"q.x == 99".as_ptr().cast_mut()).cast_const(),
                        (c"c_struct.c".as_ptr().cast_mut()).cast_const(),
                        35_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((q.y) == (20)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"q.y == 20".as_ptr().cast_mut()).cast_const(),
                        (c"c_struct.c".as_ptr().cast_mut()).cast_const(),
                        36_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut l: Line = Line {
        start: Point { x: 1, y: 2 },
        end: Point { x: 3, y: 4 },
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((l.start.x) == (1)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"l.start.x == 1".as_ptr().cast_mut()).cast_const(),
                        (c"c_struct.c".as_ptr().cast_mut()).cast_const(),
                        39_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((l.end.y) == (4)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"l.end.y == 4".as_ptr().cast_mut()).cast_const(),
                        (c"c_struct.c".as_ptr().cast_mut()).cast_const(),
                        40_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut a: Node = Node {
        value: 1,
        next: std::ptr::null_mut(),
    };
    let mut b: Node = Node {
        value: 2,
        next: (&mut a as *mut Node),
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*b.next).value) == (1)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"b.next->value == 1".as_ptr().cast_mut()).cast_const(),
                        (c"c_struct.c".as_ptr().cast_mut()).cast_const(),
                        44_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut c: Container = Container {
        inner: Inner { a: 5, b: 6 },
        color: Color_GREEN,
        count: 42,
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c.inner.a) == (5)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"c.inner.a == 5".as_ptr().cast_mut()).cast_const(),
                        (c"c_struct.c".as_ptr().cast_mut()).cast_const(),
                        47_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c.inner.b) == (6)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"c.inner.b == 6".as_ptr().cast_mut()).cast_const(),
                        (c"c_struct.c".as_ptr().cast_mut()).cast_const(),
                        48_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c.color as u32) == ((Color_GREEN as i32) as u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"c.color == GREEN".as_ptr().cast_mut()).cast_const(),
                        (c"c_struct.c".as_ptr().cast_mut()).cast_const(),
                        49_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c.count) == (42)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"c.count == 42".as_ptr().cast_mut()).cast_const(),
                        (c"c_struct.c".as_ptr().cast_mut()).cast_const(),
                        50_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut c2: Container = <Container>::default();
    c2.color = Color_BLUE;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c2.color as u32) == (2_u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"c2.color == 2".as_ptr().cast_mut()).cast_const(),
                        (c"c_struct.c".as_ptr().cast_mut()).cast_const(),
                        54_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
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
