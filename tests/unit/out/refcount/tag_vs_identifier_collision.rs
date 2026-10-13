extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type widget_enum = u32;
pub const widget_enum_MODE_IDLE: widget_enum = 0;
pub const widget_enum_MODE_ACTIVE: widget_enum = 1;
pub const widget_enum_MODE_DONE: widget_enum = 2;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct widget {
    #[offset(0)]
    pub id: i32,
    #[offset(4)]
    pub mode: widget_enum,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct point_struct {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
#[derive(VaArg, FnPtrArg, ByteRepr, DeepClone)]
#[byte_size(4)]
pub struct point {
    #[offset(0)]
    #[byte_size(4)]
    __bytes: Value<Box<[u8]>>,
}
impl point {
    pub fn whole(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
    pub fn half(this: Ptr<Self>) -> Ptr<i16> {
        this.reinterpret_cast()
    }
}
impl Default for point {
    fn default() -> Self {
        point {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 4]))),
        }
    }
}
#[derive(VaArg, FnPtrArg, ByteRepr, DeepClone)]
#[byte_size(4)]
pub struct slot_union {
    #[offset(0)]
    #[byte_size(4)]
    __bytes: Value<Box<[u8]>>,
}
impl slot_union {
    pub fn i(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
    pub fn u(this: Ptr<Self>) -> Ptr<u32> {
        this.reinterpret_cast()
    }
}
impl Default for slot_union {
    fn default() -> Self {
        slot_union {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 4]))),
        }
    }
}
pub type slot = u32;
pub const slot_SLOT_A: slot = 0;
pub const slot_SLOT_B: slot = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Inner {
    #[offset(0)]
    pub tag_field: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(4)]
    pub field: Inner,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Inner_struct {
    #[offset(0)]
    pub typedef_field: i32,
}
pub fn is_active_0(mut w: Ptr<widget>) -> i32 {
    return (((w.with(|__s| __s.mode) as u32) == ((widget_enum_MODE_ACTIVE as i32) as u32)) as i32);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let w: Value<widget> = <Value<widget>>::default();
    (*w.borrow_mut()).id = 7;
    (*w.borrow_mut()).mode = widget_enum_MODE_ACTIVE;
    {
        &(4usize);
        {
            if (({ is_active_0((w.as_pointer())) }) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"is_active(&w)"),
                        Ptr::<i8>::from_string_literal(b"tag_vs_identifier_collision.c"),
                        43_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    (*w.borrow_mut()).mode = widget_enum_MODE_DONE;
    {
        &(4usize);
        {
            if (((({ (*w.borrow()).mode } as u32) == ((widget_enum_MODE_DONE as i32) as u32))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"w.mode == MODE_DONE"),
                        Ptr::<i8>::from_string_literal(b"tag_vs_identifier_collision.c"),
                        45_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let mut p: point_struct = <point_struct>::default();
    p.x = 3;
    p.y = 4;
    {
        &(4usize);
        {
            if ((((p.x + p.y) == 7) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"p.x + p.y == 7"),
                        Ptr::<i8>::from_string_literal(b"tag_vs_identifier_collision.c"),
                        50_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let up: Value<point> = <Value<point>>::default();
    point::whole(up.as_pointer()).write(5);
    {
        &(4usize);
        {
            if ((((point::whole(up.as_pointer()).read()) == 5) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"up.whole == 5"),
                        Ptr::<i8>::from_string_literal(b"tag_vs_identifier_collision.c"),
                        54_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let b: Value<slot_union> = <Value<slot_union>>::default();
    slot_union::i(b.as_pointer()).write(9);
    {
        &(4usize);
        {
            if ((((slot_union::i(b.as_pointer()).read()) == 9) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"b.i == 9"),
                        Ptr::<i8>::from_string_literal(b"tag_vs_identifier_collision.c"),
                        58_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let mut e: slot = slot_SLOT_B;
    {
        &(4usize);
        {
            if ((((e as u32) == ((slot_SLOT_B as i32) as u32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"e == SLOT_B"),
                        Ptr::<i8>::from_string_literal(b"tag_vs_identifier_collision.c"),
                        61_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let mut inner_tag: Inner = <Inner>::default();
    inner_tag.tag_field = 11;
    {
        &(4usize);
        {
            if (((inner_tag.tag_field == 11) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"inner_tag.tag_field == 11"),
                        Ptr::<i8>::from_string_literal(b"tag_vs_identifier_collision.c"),
                        65_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let mut inner_typedef: Inner_struct = <Inner_struct>::default();
    inner_typedef.typedef_field = 22;
    {
        &(4usize);
        {
            if (((inner_typedef.typedef_field == 22) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"inner_typedef.typedef_field == 22"),
                        Ptr::<i8>::from_string_literal(b"tag_vs_identifier_collision.c"),
                        69_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let mut o: Outer = <Outer>::default();
    o.field.tag_field = 33;
    {
        &(4usize);
        {
            if (((o.field.tag_field == 33) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"o.field.tag_field == 33"),
                        Ptr::<i8>::from_string_literal(b"tag_vs_identifier_collision.c"),
                        73_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((({ (*w.borrow()).id } == 7) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"w.id == 7"),
                        Ptr::<i8>::from_string_literal(b"tag_vs_identifier_collision.c"),
                        75_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    return 0;
}
pub fn __assert_fail_1(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
