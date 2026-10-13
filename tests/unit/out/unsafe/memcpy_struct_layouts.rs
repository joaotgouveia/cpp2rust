extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct __fsid_t {
    pub __val: [i32; 2],
}
impl Clone for __fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct packed {
    pub a: i32,
    pub b: libc::c_char,
    pub c: libc::c_char,
    pub d: i16,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct reordered {
    pub a: libc::c_char,
    pub b: i32,
    pub c: libc::c_char,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct tail {
    pub a: libc::c_char,
    pub b: f64,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct nested {
    pub t: tail,
    pub c: libc::c_char,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct array {
    pub name: [libc::c_char; 3],
    pub x: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut buf: [u8; 64] = [0_u8; 64];
    let mut p: [packed; 2] = [
        packed {
            a: 1,
            b: (2 as libc::c_char),
            c: (3 as libc::c_char),
            d: 4_i16,
        },
        packed {
            a: 5,
            b: (6 as libc::c_char),
            c: (7 as libc::c_char),
            d: 8_i16,
        },
    ];
    (unsafe {
        let ___src: *const ::libc::c_void = (p.as_mut_ptr() as *const ::libc::c_void);
        let ___n: usize = ::std::mem::size_of::<[packed; 2]>();
        memcpy_0((buf.as_mut_ptr() as *mut ::libc::c_void), ___src, ___n)
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((buf[((::std::mem::size_of::<packed>() as usize).wrapping_add(4_usize))] as i32)
                == (6)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"buf[sizeof(struct packed) + 4] == 6".as_ptr().cast_mut()).cast_const(),
                        (c"memcpy_struct_layouts.c".as_ptr().cast_mut()).cast_const(),
                        42_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut p2: [packed; 2] = [<packed>::default(); 2];
    (unsafe {
        let ___dest: *mut ::libc::c_void = (p2.as_mut_ptr() as *mut ::libc::c_void);
        let ___n: usize = ::std::mem::size_of::<[packed; 2]>();
        memcpy_0(___dest, (buf.as_mut_ptr() as *const ::libc::c_void), ___n)
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((((((((((p2[(1) as usize].a) == (5)) as i32) != 0)
                && ((((p2[(1) as usize].b as i32) == (6)) as i32) != 0))
                as i32)
                != 0)
                && ((((p2[(1) as usize].c as i32) == (7)) as i32) != 0))
                as i32)
                != 0)
                && ((((p2[(1) as usize].d as i32) == (8)) as i32) != 0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"p2[1].a == 5 && p2[1].b == 6 && p2[1].c == 7 && p2[1].d == 8"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"memcpy_struct_layouts.c".as_ptr().cast_mut()).cast_const(),
                        45_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut r: [reordered; 2] = [
        reordered {
            a: (1 as libc::c_char),
            b: 2,
            c: (3 as libc::c_char),
        },
        reordered {
            a: (4 as libc::c_char),
            b: 5,
            c: (6 as libc::c_char),
        },
    ];
    (unsafe {
        let ___src: *const ::libc::c_void = (r.as_mut_ptr() as *const ::libc::c_void);
        let ___n: usize = ::std::mem::size_of::<[reordered; 2]>();
        memcpy_0((buf.as_mut_ptr() as *mut ::libc::c_void), ___src, ___n)
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((buf[((::std::mem::size_of::<reordered>() as usize).wrapping_add(8_usize))]
                as i32)
                == (6)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"buf[sizeof(struct reordered) + 8] == 6"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"memcpy_struct_layouts.c".as_ptr().cast_mut()).cast_const(),
                        49_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut r2: [reordered; 2] = [<reordered>::default(); 2];
    (unsafe {
        let ___dest: *mut ::libc::c_void = (r2.as_mut_ptr() as *mut ::libc::c_void);
        let ___n: usize = ::std::mem::size_of::<[reordered; 2]>();
        memcpy_0(___dest, (buf.as_mut_ptr() as *const ::libc::c_void), ___n)
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((((((((r2[(1) as usize].a as i32) == (4)) as i32) != 0)
                && ((((r2[(1) as usize].b) == (5)) as i32) != 0)) as i32)
                != 0)
                && ((((r2[(1) as usize].c as i32) == (6)) as i32) != 0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"r2[1].a == 4 && r2[1].b == 5 && r2[1].c == 6"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"memcpy_struct_layouts.c".as_ptr().cast_mut()).cast_const(),
                        52_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut n: [nested; 2] = [
        nested {
            t: tail {
                a: (1 as libc::c_char),
                b: 2.5E+0,
            },
            c: (3 as libc::c_char),
        },
        nested {
            t: tail {
                a: (4 as libc::c_char),
                b: 5.5E+0,
            },
            c: (6 as libc::c_char),
        },
    ];
    (unsafe {
        let ___src: *const ::libc::c_void = (n.as_mut_ptr() as *const ::libc::c_void);
        let ___n: usize = ::std::mem::size_of::<[nested; 2]>();
        memcpy_0((buf.as_mut_ptr() as *mut ::libc::c_void), ___src, ___n)
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((buf[((::std::mem::size_of::<nested>() as usize).wrapping_add(16_usize))] as i32)
                == (6)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"buf[sizeof(struct nested) + 16] == 6".as_ptr().cast_mut()).cast_const(),
                        (c"memcpy_struct_layouts.c".as_ptr().cast_mut()).cast_const(),
                        56_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut n2: [nested; 2] = [<nested>::default(); 2];
    (unsafe {
        let ___dest: *mut ::libc::c_void = (n2.as_mut_ptr() as *mut ::libc::c_void);
        let ___n: usize = ::std::mem::size_of::<[nested; 2]>();
        memcpy_0(___dest, (buf.as_mut_ptr() as *const ::libc::c_void), ___n)
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((((((((n2[(1) as usize].t.a as i32) == (4)) as i32) != 0)
                && ((((n2[(1) as usize].t.b) == (5.5E+0)) as i32) != 0))
                as i32)
                != 0)
                && ((((n2[(1) as usize].c as i32) == (6)) as i32) != 0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"n2[1].t.a == 4 && n2[1].t.b == 5.5 && n2[1].c == 6"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"memcpy_struct_layouts.c".as_ptr().cast_mut()).cast_const(),
                        59_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut a: [array; 2] = [
        array {
            name: std::mem::transmute(*b"ab\0"),
            x: 1,
        },
        array {
            name: std::mem::transmute(*b"cd\0"),
            x: 2,
        },
    ];
    (unsafe {
        let ___src: *const ::libc::c_void = (a.as_mut_ptr() as *const ::libc::c_void);
        let ___n: usize = ::std::mem::size_of::<[array; 2]>();
        memcpy_0((buf.as_mut_ptr() as *mut ::libc::c_void), ___src, ___n)
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((buf[((::std::mem::size_of::<array>() as usize).wrapping_add(1_usize))] as i32)
                == ('d' as i32)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"buf[sizeof(struct array) + 1] == \'d\'"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"memcpy_struct_layouts.c".as_ptr().cast_mut()).cast_const(),
                        63_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut a2: [array; 2] = [<array>::default(); 2];
    (unsafe {
        let ___dest: *mut ::libc::c_void = (a2.as_mut_ptr() as *mut ::libc::c_void);
        let ___n: usize = ::std::mem::size_of::<[array; 2]>();
        memcpy_0(___dest, (buf.as_mut_ptr() as *const ::libc::c_void), ___n)
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((((((((a2[(1) as usize].name[(1) as usize] as i32) == ('d' as i32)) as i32) != 0)
                && ((((a2[(1) as usize].name[(2) as usize] as i32) == (0)) as i32) != 0))
                as i32)
                != 0)
                && ((((a2[(1) as usize].x) == (2)) as i32) != 0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"a2[1].name[1] == \'d\' && a2[1].name[2] == 0 && a2[1].x == 2"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"memcpy_struct_layouts.c".as_ptr().cast_mut()).cast_const(),
                        66_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
pub unsafe fn memcpy_0(
    mut __dest: *mut ::libc::c_void,
    mut __src: *const ::libc::c_void,
    mut __n: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn __assert_fail_1(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
