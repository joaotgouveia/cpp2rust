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
pub struct __mbstate_t {
    pub __count: i32,
    pub __value: anon_0,
}
impl Clone for __mbstate_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __mbstate_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct anon_0 {
    pub __wch: u32,
    pub __wchb: [libc::c_char; 4],
}
impl Clone for anon_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _G_fpos_t {
    pub __pos: i64,
    pub __state: __mbstate_t,
}
impl Clone for _G_fpos_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _G_fpos_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _G_fpos64_t {
    pub __pos: i64,
    pub __state: __mbstate_t,
}
impl Clone for _G_fpos64_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _G_fpos64_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: i64,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut ::libc::c_void,
    pub _offset: i64,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::libc::c_void,
    pub __pad5: usize,
    pub _mode: i32,
    pub _unused2: [libc::c_char; 20],
}
impl Clone for _IO_FILE {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _IO_FILE {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _IO_cookie_io_functions_t {
    pub read: Option<unsafe fn(*mut ::libc::c_void, *mut libc::c_char, usize) -> i64>,
    pub write: Option<unsafe fn(*mut ::libc::c_void, *const libc::c_char, usize) -> i64>,
    pub seek: Option<unsafe fn(*mut ::libc::c_void, *mut i64, i32) -> i32>,
    pub close: Option<unsafe fn(*mut ::libc::c_void) -> i32>,
}
impl Clone for _IO_cookie_io_functions_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _IO_cookie_io_functions_t {
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
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut buf: [libc::c_char; 32] = [(0 as libc::c_char); 32];
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                let ___s: *mut libc::c_char = buf.as_mut_ptr();
                let ___maxlen: usize = ::std::mem::size_of::<[libc::c_char; 32]>();
                snprintf_1(
                    ___s,
                    ___maxlen,
                    (c"x=%d y=%u".as_ptr().cast_mut()).cast_const(),
                    &[(-3_i32).into(), (7_u32).into()],
                )
            }) == (8)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"snprintf(buf, sizeof(buf), \"x=%d y=%u\", -3, 7u) == 8"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        7_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcmp_3(
                    (buf.as_mut_ptr()).cast_const(),
                    (c"x=-3 y=7".as_ptr().cast_mut()).cast_const(),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcmp(buf, \"x=-3 y=7\") == 0".as_ptr().cast_mut()).cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        8_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                snprintf_1(
                    buf.as_mut_ptr(),
                    4_usize,
                    (c"%s".as_ptr().cast_mut()).cast_const(),
                    &[(c"hello".as_ptr().cast_mut()).into()],
                )
            }) == (5)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"snprintf(buf, 4, \"%s\", \"hello\") == 5"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        9_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcmp_3(
                    (buf.as_mut_ptr()).cast_const(),
                    (c"hel".as_ptr().cast_mut()).cast_const(),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcmp(buf, \"hel\") == 0".as_ptr().cast_mut()).cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        10_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                let ___s: *mut libc::c_char = buf.as_mut_ptr();
                let ___maxlen: usize = ::std::mem::size_of::<[libc::c_char; 32]>();
                snprintf_1(
                    ___s,
                    ___maxlen,
                    (c"%05d|%x|%X".as_ptr().cast_mut()).cast_const(),
                    &[(42).into(), (255).into(), (255).into()],
                )
            }) == (11)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"snprintf(buf, sizeof(buf), \"%05d|%x|%X\", 42, 255, 255) == 11"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        11_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcmp_3(
                    (buf.as_mut_ptr()).cast_const(),
                    (c"00042|ff|FF".as_ptr().cast_mut()).cast_const(),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcmp(buf, \"00042|ff|FF\") == 0".as_ptr().cast_mut()).cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        12_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                let ___s: *mut libc::c_char = buf.as_mut_ptr();
                let ___maxlen: usize = ::std::mem::size_of::<[libc::c_char; 32]>();
                snprintf_1(
                    ___s,
                    ___maxlen,
                    (c"%.2f".as_ptr().cast_mut()).cast_const(),
                    &[(3.1415899999999999E+0).into()],
                )
            }) == (4)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"snprintf(buf, sizeof(buf), \"%.2f\", 3.14159) == 4"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        13_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcmp_3(
                    (buf.as_mut_ptr()).cast_const(),
                    (c"3.14".as_ptr().cast_mut()).cast_const(),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcmp(buf, \"3.14\") == 0".as_ptr().cast_mut()).cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        14_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                let ___s: *mut libc::c_char = buf.as_mut_ptr();
                let ___maxlen: usize = ::std::mem::size_of::<[libc::c_char; 32]>();
                snprintf_1(
                    ___s,
                    ___maxlen,
                    (c"%-6s|".as_ptr().cast_mut()).cast_const(),
                    &[(c"ab".as_ptr().cast_mut()).into()],
                )
            }) == (7)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"snprintf(buf, sizeof(buf), \"%-6s|\", \"ab\") == 7"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        15_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcmp_3(
                    (buf.as_mut_ptr()).cast_const(),
                    (c"ab    |".as_ptr().cast_mut()).cast_const(),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcmp(buf, \"ab    |\") == 0".as_ptr().cast_mut()).cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        16_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                let ___s: *mut libc::c_char = buf.as_mut_ptr();
                let ___maxlen: usize = ::std::mem::size_of::<[libc::c_char; 32]>();
                snprintf_1(
                    ___s,
                    ___maxlen,
                    (c"%c%%".as_ptr().cast_mut()).cast_const(),
                    &[(65).into()],
                )
            }) == (2)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"snprintf(buf, sizeof(buf), \"%c%%\", 65) == 2"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        17_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcmp_3(
                    (buf.as_mut_ptr()).cast_const(),
                    (c"A%".as_ptr().cast_mut()).cast_const(),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcmp(buf, \"A%\") == 0".as_ptr().cast_mut()).cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        18_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                let ___s: *mut libc::c_char = buf.as_mut_ptr();
                let ___maxlen: usize = ::std::mem::size_of::<[libc::c_char; 32]>();
                snprintf_1(
                    ___s,
                    ___maxlen,
                    (c"%+d % d".as_ptr().cast_mut()).cast_const(),
                    &[(5).into(), (5).into()],
                )
            }) == (5)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"snprintf(buf, sizeof(buf), \"%+d % d\", 5, 5) == 5"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        19_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcmp_3(
                    (buf.as_mut_ptr()).cast_const(),
                    (c"+5  5".as_ptr().cast_mut()).cast_const(),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcmp(buf, \"+5  5\") == 0".as_ptr().cast_mut()).cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        20_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                let ___s: *mut libc::c_char = buf.as_mut_ptr();
                let ___maxlen: usize = ::std::mem::size_of::<[libc::c_char; 32]>();
                snprintf_1(
                    ___s,
                    ___maxlen,
                    (c"%ld %lu %zu".as_ptr().cast_mut()).cast_const(),
                    &[(-1_i64).into(), (1_u64).into(), (9 as usize).into()],
                )
            }) == (6)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"snprintf(buf, sizeof(buf), \"%ld %lu %zu\", -1L, 1UL, (size_t)9) == 6"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        21_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcmp_3(
                    (buf.as_mut_ptr()).cast_const(),
                    (c"-1 1 9".as_ptr().cast_mut()).cast_const(),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcmp(buf, \"-1 1 9\") == 0".as_ptr().cast_mut()).cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        22_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                let ___s: *mut libc::c_char = buf.as_mut_ptr();
                let ___maxlen: usize = ::std::mem::size_of::<[libc::c_char; 32]>();
                snprintf_1(
                    ___s,
                    ___maxlen,
                    (c"%e".as_ptr().cast_mut()).cast_const(),
                    &[(1.2345678E+3).into()],
                )
            }) == (12)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"snprintf(buf, sizeof(buf), \"%e\", 1234.5678) == 12"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        23_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcmp_3(
                    (buf.as_mut_ptr()).cast_const(),
                    (c"1.234568e+03".as_ptr().cast_mut()).cast_const(),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcmp(buf, \"1.234568e+03\") == 0".as_ptr().cast_mut()).cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        24_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                let ___s: *mut libc::c_char = buf.as_mut_ptr();
                let ___maxlen: usize = ::std::mem::size_of::<[libc::c_char; 32]>();
                snprintf_1(
                    ___s,
                    ___maxlen,
                    (c"%g".as_ptr().cast_mut()).cast_const(),
                    &[(1.234567E+6).into()],
                )
            }) == (11)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"snprintf(buf, sizeof(buf), \"%g\", 1234567.0) == 11"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        25_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcmp_3(
                    (buf.as_mut_ptr()).cast_const(),
                    (c"1.23457e+06".as_ptr().cast_mut()).cast_const(),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcmp(buf, \"1.23457e+06\") == 0".as_ptr().cast_mut()).cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        26_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    buf[(0) as usize] = (('Z' as i32) as libc::c_char);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                snprintf_1(
                    buf.as_mut_ptr(),
                    0_usize,
                    (c"%d".as_ptr().cast_mut()).cast_const(),
                    &[(123).into()],
                )
            }) == (3)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"snprintf(buf, 0, \"%d\", 123) == 3".as_ptr().cast_mut()).cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        28_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((buf[(0) as usize] as i32) == ('Z' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"buf[0] == \'Z\'".as_ptr().cast_mut()).cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        29_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut fmt: [libc::c_char; 8] = [(0 as libc::c_char); 8];
    fmt[(0) as usize] = (('%' as i32) as libc::c_char);
    fmt[(1) as usize] = (('5' as i32) as libc::c_char);
    fmt[(2) as usize] = (('.' as i32) as libc::c_char);
    fmt[(3) as usize] = (('1' as i32) as libc::c_char);
    fmt[(4) as usize] = (('f' as i32) as libc::c_char);
    fmt[(5) as usize] = (0 as libc::c_char);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                let ___s: *mut libc::c_char = buf.as_mut_ptr();
                let ___maxlen: usize = ::std::mem::size_of::<[libc::c_char; 32]>();
                snprintf_1(
                    ___s,
                    ___maxlen,
                    (fmt.as_mut_ptr()).cast_const(),
                    &[(3.2599999999999998E+0).into()],
                )
            }) == (5)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"snprintf(buf, sizeof(buf), fmt, 3.26) == 5"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        37_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcmp_3(
                    (buf.as_mut_ptr()).cast_const(),
                    (c"  3.3".as_ptr().cast_mut()).cast_const(),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcmp(buf, \"  3.3\") == 0".as_ptr().cast_mut()).cast_const(),
                        (c"snprintf.c".as_ptr().cast_mut()).cast_const(),
                        38_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_wide_data;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_codecvt;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_marker;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
pub unsafe fn snprintf_1(
    mut __s: *mut libc::c_char,
    mut __maxlen: usize,
    mut __format: *const libc::c_char,
    __args: &[VaArg],
) -> i32 {
    unimplemented!()
}
pub unsafe fn __assert_fail_2(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn strcmp_3(mut __s1: *const libc::c_char, mut __s2: *const libc::c_char) -> i32 {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
