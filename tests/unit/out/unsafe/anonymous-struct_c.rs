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
pub struct Named {
    pub a: i32,
    pub b: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_0 {
    pub c: i32,
    pub d: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_1 {
    pub g: i32,
    pub h: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_2 {
    pub e: i32,
    pub f: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_4 {
    pub j: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_5 {
    pub k: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_3 {
    pub i: i32,
    pub inner_named: anon_4,
    pub anon_5: anon_5,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer {
    pub named: Named,
    pub anon0: anon_0,
    pub anon1: anon_1,
    pub anon_2: anon_2,
    pub anon_3: anon_3,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut o: Outer = Outer {
        named: Named { a: 0, b: 0_i32 },
        anon0: <anon_0>::default(),
        anon1: <anon_1>::default(),
        anon_2: <anon_2>::default(),
        anon_3: <anon_3>::default(),
    };
    o.named.a = 1;
    o.named.b = 2;
    o.anon0.c = 3;
    o.anon0.d = 4;
    o.anon1.g = 5;
    o.anon1.h = 6;
    o.anon_2.e = 7;
    o.anon_2.f = 8;
    o.anon_3.i = 9;
    o.anon_3.inner_named.j = 10;
    o.anon_3.anon_5.k = 11;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((o.named.a) == (1)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_6(
                        (c"o.named.a == 1".as_ptr().cast_mut()).cast_const(),
                        (c"anonymous-struct_c.c".as_ptr().cast_mut()).cast_const(),
                        50_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((o.named.b) == (2)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_6(
                        (c"o.named.b == 2".as_ptr().cast_mut()).cast_const(),
                        (c"anonymous-struct_c.c".as_ptr().cast_mut()).cast_const(),
                        51_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((o.anon0.c) == (3)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_6(
                        (c"o.anon0.c == 3".as_ptr().cast_mut()).cast_const(),
                        (c"anonymous-struct_c.c".as_ptr().cast_mut()).cast_const(),
                        52_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((o.anon0.d) == (4)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_6(
                        (c"o.anon0.d == 4".as_ptr().cast_mut()).cast_const(),
                        (c"anonymous-struct_c.c".as_ptr().cast_mut()).cast_const(),
                        53_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((o.anon1.g) == (5)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_6(
                        (c"o.anon1.g == 5".as_ptr().cast_mut()).cast_const(),
                        (c"anonymous-struct_c.c".as_ptr().cast_mut()).cast_const(),
                        54_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((o.anon1.h) == (6)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_6(
                        (c"o.anon1.h == 6".as_ptr().cast_mut()).cast_const(),
                        (c"anonymous-struct_c.c".as_ptr().cast_mut()).cast_const(),
                        55_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((o.anon_2.e) == (7)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_6(
                        (c"o.e == 7".as_ptr().cast_mut()).cast_const(),
                        (c"anonymous-struct_c.c".as_ptr().cast_mut()).cast_const(),
                        56_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((o.anon_2.f) == (8)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_6(
                        (c"o.f == 8".as_ptr().cast_mut()).cast_const(),
                        (c"anonymous-struct_c.c".as_ptr().cast_mut()).cast_const(),
                        57_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((o.anon_3.i) == (9)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_6(
                        (c"o.i == 9".as_ptr().cast_mut()).cast_const(),
                        (c"anonymous-struct_c.c".as_ptr().cast_mut()).cast_const(),
                        58_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((o.anon_3.inner_named.j) == (10)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_6(
                        (c"o.inner_named.j == 10".as_ptr().cast_mut()).cast_const(),
                        (c"anonymous-struct_c.c".as_ptr().cast_mut()).cast_const(),
                        59_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((o.anon_3.anon_5.k) == (11)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_6(
                        (c"o.k == 11".as_ptr().cast_mut()).cast_const(),
                        (c"anonymous-struct_c.c".as_ptr().cast_mut()).cast_const(),
                        60_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut s: anon_7 = <anon_7>::default();
    s.x = 1;
    s.z = 2;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ({
                s.x = 1;
                s.x
            } != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_6(
                        (c"s.x = 1".as_ptr().cast_mut()).cast_const(),
                        (c"anonymous-struct_c.c".as_ptr().cast_mut()).cast_const(),
                        70_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ({
                s.z = 2;
                s.z
            } != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_6(
                        (c"s.z = 2".as_ptr().cast_mut()).cast_const(),
                        (c"anonymous-struct_c.c".as_ptr().cast_mut()).cast_const(),
                        71_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_7 {
    pub x: i32,
    pub z: i32,
}
pub unsafe fn __assert_fail_6(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
