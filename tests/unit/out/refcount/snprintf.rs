extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __fsid_t {
    #[offset(0)]
    #[byte_size(8)]
    pub __val: Value<Box<[i32]>>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __mbstate_t {
    #[offset(0)]
    pub __count: i32,
    #[offset(4)]
    #[byte_size(4)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct anon_0 {
    #[offset(0)]
    pub __wch: u32,
    #[offset(0)]
    #[byte_size(4)]
    pub __wchb: Value<Box<[i8]>>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _G_fpos_t {
    #[offset(0)]
    pub __pos: i64,
    #[offset(8)]
    #[byte_size(8)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _G_fpos64_t {
    #[offset(0)]
    pub __pos: i64,
    #[offset(8)]
    #[byte_size(8)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(216)]
pub struct _IO_FILE {
    #[offset(0)]
    pub _flags: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub _IO_read_ptr: Ptr<i8>,
    #[offset(16)]
    #[byte_size(8)]
    pub _IO_read_end: Ptr<i8>,
    #[offset(24)]
    #[byte_size(8)]
    pub _IO_read_base: Ptr<i8>,
    #[offset(32)]
    #[byte_size(8)]
    pub _IO_write_base: Ptr<i8>,
    #[offset(40)]
    #[byte_size(8)]
    pub _IO_write_ptr: Ptr<i8>,
    #[offset(48)]
    #[byte_size(8)]
    pub _IO_write_end: Ptr<i8>,
    #[offset(56)]
    #[byte_size(8)]
    pub _IO_buf_base: Ptr<i8>,
    #[offset(64)]
    #[byte_size(8)]
    pub _IO_buf_end: Ptr<i8>,
    #[offset(72)]
    #[byte_size(8)]
    pub _IO_save_base: Ptr<i8>,
    #[offset(80)]
    #[byte_size(8)]
    pub _IO_backup_base: Ptr<i8>,
    #[offset(88)]
    #[byte_size(8)]
    pub _IO_save_end: Ptr<i8>,
    #[offset(96)]
    #[byte_size(8)]
    pub _markers: Ptr<_IO_marker>,
    #[offset(104)]
    #[byte_size(8)]
    pub _chain: Ptr<_IO_FILE>,
    #[offset(112)]
    pub _fileno: i32,
    #[offset(116)]
    pub _flags2: i32,
    #[offset(120)]
    pub _old_offset: i64,
    #[offset(128)]
    pub _cur_column: u16,
    #[offset(130)]
    pub _vtable_offset: i8,
    #[offset(131)]
    #[byte_size(1)]
    pub _shortbuf: Value<Box<[i8]>>,
    #[offset(136)]
    #[byte_size(8)]
    pub _lock: AnyPtr,
    #[offset(144)]
    pub _offset: i64,
    #[offset(152)]
    #[byte_size(8)]
    pub _codecvt: Ptr<_IO_codecvt>,
    #[offset(160)]
    #[byte_size(8)]
    pub _wide_data: Ptr<_IO_wide_data>,
    #[offset(168)]
    #[byte_size(8)]
    pub _freeres_list: Ptr<_IO_FILE>,
    #[offset(176)]
    #[byte_size(8)]
    pub _freeres_buf: AnyPtr,
    #[offset(184)]
    pub __pad5: usize,
    #[offset(192)]
    pub _mode: i32,
    #[offset(196)]
    #[byte_size(20)]
    pub _unused2: Value<Box<[i8]>>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct _IO_cookie_io_functions_t {
    #[offset(0)]
    #[byte_size(8)]
    pub read: FnPtr<fn(AnyPtr, Ptr<i8>, usize) -> i64>,
    #[offset(8)]
    #[byte_size(8)]
    pub write: FnPtr<fn(AnyPtr, Ptr<i8>, usize) -> i64>,
    #[offset(16)]
    #[byte_size(8)]
    pub seek: FnPtr<fn(AnyPtr, Ptr<i64>, i32) -> i32>,
    #[offset(24)]
    #[byte_size(8)]
    pub close: FnPtr<fn(AnyPtr) -> i32>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(232)]
pub struct __locale_struct {
    #[offset(0)]
    #[byte_size(104)]
    pub __locales: Value<Box<[Ptr<__locale_data>]>>,
    #[offset(104)]
    #[byte_size(8)]
    pub __ctype_b: Ptr<u16>,
    #[offset(112)]
    #[byte_size(8)]
    pub __ctype_tolower: Ptr<i32>,
    #[offset(120)]
    #[byte_size(8)]
    pub __ctype_toupper: Ptr<i32>,
    #[offset(128)]
    #[byte_size(104)]
    pub __names: Value<Box<[Ptr<i8>]>>,
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
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new((0..32).map(|_| 0_i8).collect::<Box<[i8]>>()));
    {
        &(4usize);
        {
            if (((({
                let ___s: Ptr<i8> = (buf.as_pointer() as Ptr<i8>);
                let ___maxlen: usize = 32usize;
                snprintf_1(
                    ___s,
                    ___maxlen,
                    Ptr::<i8>::from_string_literal(b"x=%d y=%u"),
                    &[(-3_i32).into(), (7_u32).into()],
                )
            }) == 8) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(
                            b"snprintf(buf, sizeof(buf), \"x=%d y=%u\", -3, 7u) == 8",
                        ),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        7_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                strcmp_3(
                    (buf.as_pointer() as Ptr<i8>),
                    Ptr::<i8>::from_string_literal(b"x=-3 y=7"),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcmp(buf, \"x=-3 y=7\") == 0"),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        8_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                snprintf_1(
                    (buf.as_pointer() as Ptr<i8>),
                    4_usize,
                    Ptr::<i8>::from_string_literal(b"%s"),
                    &[(Ptr::<i8>::from_string_literal(b"hello")).into()],
                )
            }) == 5) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"snprintf(buf, 4, \"%s\", \"hello\") == 5"),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        9_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                strcmp_3(
                    (buf.as_pointer() as Ptr<i8>),
                    Ptr::<i8>::from_string_literal(b"hel"),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcmp(buf, \"hel\") == 0"),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        10_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                let ___s: Ptr<i8> = (buf.as_pointer() as Ptr<i8>);
                let ___maxlen: usize = 32usize;
                snprintf_1(
                    ___s,
                    ___maxlen,
                    Ptr::<i8>::from_string_literal(b"%05d|%x|%X"),
                    &[(42).into(), (255).into(), (255).into()],
                )
            }) == 11) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(
                            b"snprintf(buf, sizeof(buf), \"%05d|%x|%X\", 42, 255, 255) == 11",
                        ),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        11_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                strcmp_3(
                    (buf.as_pointer() as Ptr<i8>),
                    Ptr::<i8>::from_string_literal(b"00042|ff|FF"),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcmp(buf, \"00042|ff|FF\") == 0"),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        12_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                let ___s: Ptr<i8> = (buf.as_pointer() as Ptr<i8>);
                let ___maxlen: usize = 32usize;
                snprintf_1(
                    ___s,
                    ___maxlen,
                    Ptr::<i8>::from_string_literal(b"%.2f"),
                    &[(3.1415899999999999E+0).into()],
                )
            }) == 4) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(
                            b"snprintf(buf, sizeof(buf), \"%.2f\", 3.14159) == 4",
                        ),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        13_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                strcmp_3(
                    (buf.as_pointer() as Ptr<i8>),
                    Ptr::<i8>::from_string_literal(b"3.14"),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcmp(buf, \"3.14\") == 0"),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        14_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                let ___s: Ptr<i8> = (buf.as_pointer() as Ptr<i8>);
                let ___maxlen: usize = 32usize;
                snprintf_1(
                    ___s,
                    ___maxlen,
                    Ptr::<i8>::from_string_literal(b"%-6s|"),
                    &[(Ptr::<i8>::from_string_literal(b"ab")).into()],
                )
            }) == 7) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(
                            b"snprintf(buf, sizeof(buf), \"%-6s|\", \"ab\") == 7",
                        ),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        15_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                strcmp_3(
                    (buf.as_pointer() as Ptr<i8>),
                    Ptr::<i8>::from_string_literal(b"ab    |"),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcmp(buf, \"ab    |\") == 0"),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        16_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                let ___s: Ptr<i8> = (buf.as_pointer() as Ptr<i8>);
                let ___maxlen: usize = 32usize;
                snprintf_1(
                    ___s,
                    ___maxlen,
                    Ptr::<i8>::from_string_literal(b"%c%%"),
                    &[(65).into()],
                )
            }) == 2) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(
                            b"snprintf(buf, sizeof(buf), \"%c%%\", 65) == 2",
                        ),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        17_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                strcmp_3(
                    (buf.as_pointer() as Ptr<i8>),
                    Ptr::<i8>::from_string_literal(b"A%"),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcmp(buf, \"A%\") == 0"),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        18_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                let ___s: Ptr<i8> = (buf.as_pointer() as Ptr<i8>);
                let ___maxlen: usize = 32usize;
                snprintf_1(
                    ___s,
                    ___maxlen,
                    Ptr::<i8>::from_string_literal(b"%+d % d"),
                    &[(5).into(), (5).into()],
                )
            }) == 5) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(
                            b"snprintf(buf, sizeof(buf), \"%+d % d\", 5, 5) == 5",
                        ),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        19_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                strcmp_3(
                    (buf.as_pointer() as Ptr<i8>),
                    Ptr::<i8>::from_string_literal(b"+5  5"),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcmp(buf, \"+5  5\") == 0"),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        20_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                let ___s: Ptr<i8> = (buf.as_pointer() as Ptr<i8>);
                let ___maxlen: usize = 32usize;
                snprintf_1(
                    ___s,
                    ___maxlen,
                    Ptr::<i8>::from_string_literal(b"%ld %lu %zu"),
                    &[(-1_i64).into(), (1_u64).into(), (9 as usize).into()],
                )
            }) == 6) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2 ( Ptr::<i8 >::from_string_literal(b"snprintf(buf, sizeof(buf), \"%ld %lu %zu\", -1L, 1UL, (size_t)9) == 6" )  , Ptr::<i8 >::from_string_literal(b"snprintf.c" )  , 21_u32  , Ptr::<i8 >::from_string_literal(b"int main()" )  , )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                strcmp_3(
                    (buf.as_pointer() as Ptr<i8>),
                    Ptr::<i8>::from_string_literal(b"-1 1 9"),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcmp(buf, \"-1 1 9\") == 0"),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        22_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                let ___s: Ptr<i8> = (buf.as_pointer() as Ptr<i8>);
                let ___maxlen: usize = 32usize;
                snprintf_1(
                    ___s,
                    ___maxlen,
                    Ptr::<i8>::from_string_literal(b"%e"),
                    &[(1.2345678E+3).into()],
                )
            }) == 12) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(
                            b"snprintf(buf, sizeof(buf), \"%e\", 1234.5678) == 12",
                        ),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        23_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                strcmp_3(
                    (buf.as_pointer() as Ptr<i8>),
                    Ptr::<i8>::from_string_literal(b"1.234568e+03"),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcmp(buf, \"1.234568e+03\") == 0"),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        24_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                let ___s: Ptr<i8> = (buf.as_pointer() as Ptr<i8>);
                let ___maxlen: usize = 32usize;
                snprintf_1(
                    ___s,
                    ___maxlen,
                    Ptr::<i8>::from_string_literal(b"%g"),
                    &[(1.234567E+6).into()],
                )
            }) == 11) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(
                            b"snprintf(buf, sizeof(buf), \"%g\", 1234567.0) == 11",
                        ),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        25_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                strcmp_3(
                    (buf.as_pointer() as Ptr<i8>),
                    Ptr::<i8>::from_string_literal(b"1.23457e+06"),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcmp(buf, \"1.23457e+06\") == 0"),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        26_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    (*buf.borrow_mut())[(0) as usize] = (('Z' as i32) as i8);
    {
        &(4usize);
        {
            if (((({
                snprintf_1(
                    (buf.as_pointer() as Ptr<i8>),
                    0_usize,
                    Ptr::<i8>::from_string_literal(b"%d"),
                    &[(123).into()],
                )
            }) == 3) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"snprintf(buf, 0, \"%d\", 123) == 3"),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        28_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((((*buf.borrow())[(0) as usize] as i32) == ('Z' as i32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"buf[0] == \'Z\'"),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        29_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let fmt: Value<Box<[i8]>> = Rc::new(RefCell::new((0..8).map(|_| 0_i8).collect::<Box<[i8]>>()));
    (*fmt.borrow_mut())[(0) as usize] = (('%' as i32) as i8);
    (*fmt.borrow_mut())[(1) as usize] = (('5' as i32) as i8);
    (*fmt.borrow_mut())[(2) as usize] = (('.' as i32) as i8);
    (*fmt.borrow_mut())[(3) as usize] = (('1' as i32) as i8);
    (*fmt.borrow_mut())[(4) as usize] = (('f' as i32) as i8);
    (*fmt.borrow_mut())[(5) as usize] = 0_i8;
    {
        &(4usize);
        {
            if (((({
                let ___s: Ptr<i8> = (buf.as_pointer() as Ptr<i8>);
                let ___maxlen: usize = 32usize;
                snprintf_1(
                    ___s,
                    ___maxlen,
                    (fmt.as_pointer() as Ptr<i8>),
                    &[(3.2599999999999998E+0).into()],
                )
            }) == 5) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(
                            b"snprintf(buf, sizeof(buf), fmt, 3.26) == 5",
                        ),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        37_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                strcmp_3(
                    (buf.as_pointer() as Ptr<i8>),
                    Ptr::<i8>::from_string_literal(b"  3.3"),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcmp(buf, \"  3.3\") == 0"),
                        Ptr::<i8>::from_string_literal(b"snprintf.c"),
                        38_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_wide_data;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_codecvt;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_marker;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __locale_data;
pub fn snprintf_1(
    mut __s: Ptr<i8>,
    mut __maxlen: usize,
    mut __format: Ptr<i8>,
    __args: &[VaArg],
) -> i32 {
    unimplemented!()
}
pub fn __assert_fail_2(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn strcmp_3(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>) -> i32 {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
