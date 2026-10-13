extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type widget_enum = u32;
pub const widget_enum_MODE_IDLE: widget_enum = 0;
pub const widget_enum_MODE_ACTIVE: widget_enum = 1;
pub const widget_enum_MODE_DONE: widget_enum = 2;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct widget {
    pub id: i32,
    pub mode: widget_enum,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct point_struct {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union point {
    pub whole: i32,
    pub half: i16,
}
impl Default for point {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union slot_union {
    pub i: i32,
    pub u: u32,
}
impl Default for slot_union {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub type slot = u32;
pub const slot_SLOT_A: slot = 0;
pub const slot_SLOT_B: slot = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Inner {
    pub tag_field: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer {
    pub field: Inner,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Inner_struct {
    pub typedef_field: i32,
}
pub unsafe fn is_active_0(mut w: *mut widget) -> i32 {
    return ((((*w).mode as u32) == ((widget_enum_MODE_ACTIVE as i32) as u32)) as i32);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut w: widget = <widget>::default();
    w.id = 7;
    w.mode = widget_enum_MODE_ACTIVE;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((unsafe { is_active_0((&mut w as *mut widget)) }) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"is_active(&w)".as_ptr().cast_mut()).cast_const(),
                        (c"tag_vs_identifier_collision.c".as_ptr().cast_mut()).cast_const(),
                        43_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    w.mode = widget_enum_MODE_DONE;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((w.mode as u32) == ((widget_enum_MODE_DONE as i32) as u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"w.mode == MODE_DONE".as_ptr().cast_mut()).cast_const(),
                        (c"tag_vs_identifier_collision.c".as_ptr().cast_mut()).cast_const(),
                        45_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut p: point_struct = <point_struct>::default();
    p.x = 3;
    p.y = 4;
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((p.x) + (p.y)) == (7)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"p.x + p.y == 7".as_ptr().cast_mut()).cast_const(),
                        (c"tag_vs_identifier_collision.c".as_ptr().cast_mut()).cast_const(),
                        50_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut up: point = <point>::default();
    up.whole = 5;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((up.whole) == (5)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"up.whole == 5".as_ptr().cast_mut()).cast_const(),
                        (c"tag_vs_identifier_collision.c".as_ptr().cast_mut()).cast_const(),
                        54_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut b: slot_union = <slot_union>::default();
    b.i = 9;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((b.i) == (9)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"b.i == 9".as_ptr().cast_mut()).cast_const(),
                        (c"tag_vs_identifier_collision.c".as_ptr().cast_mut()).cast_const(),
                        58_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut e: slot = slot_SLOT_B;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((e as u32) == ((slot_SLOT_B as i32) as u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"e == SLOT_B".as_ptr().cast_mut()).cast_const(),
                        (c"tag_vs_identifier_collision.c".as_ptr().cast_mut()).cast_const(),
                        61_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut inner_tag: Inner = <Inner>::default();
    inner_tag.tag_field = 11;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((inner_tag.tag_field) == (11)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"inner_tag.tag_field == 11".as_ptr().cast_mut()).cast_const(),
                        (c"tag_vs_identifier_collision.c".as_ptr().cast_mut()).cast_const(),
                        65_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut inner_typedef: Inner_struct = <Inner_struct>::default();
    inner_typedef.typedef_field = 22;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((inner_typedef.typedef_field) == (22)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"inner_typedef.typedef_field == 22".as_ptr().cast_mut()).cast_const(),
                        (c"tag_vs_identifier_collision.c".as_ptr().cast_mut()).cast_const(),
                        69_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut o: Outer = <Outer>::default();
    o.field.tag_field = 33;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((o.field.tag_field) == (33)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"o.field.tag_field == 33".as_ptr().cast_mut()).cast_const(),
                        (c"tag_vs_identifier_collision.c".as_ptr().cast_mut()).cast_const(),
                        73_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((w.id) == (7)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"w.id == 7".as_ptr().cast_mut()).cast_const(),
                        (c"tag_vs_identifier_collision.c".as_ptr().cast_mut()).cast_const(),
                        75_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    return 0;
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
