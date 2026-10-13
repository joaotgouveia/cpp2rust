extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Line {
    #[offset(0)]
    #[byte_size(8)]
    pub start: Point,
    #[offset(8)]
    #[byte_size(8)]
    pub end: Point,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Node {
    #[offset(0)]
    pub value: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub next: Ptr<Node>,
}
pub type Color = u32;
pub const Color_RED: Color = 0;
pub const Color_GREEN: Color = 1;
pub const Color_BLUE: Color = 2;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Inner {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Container {
    #[offset(0)]
    #[byte_size(8)]
    pub inner: Inner,
    #[offset(8)]
    pub color: Color,
    #[offset(12)]
    pub count: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut p: Point = Point { x: 10, y: 20 };
    {
        &(4usize);
        {
            if (((p.x == 10) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"p.x == 10"),
                        Ptr::<i8>::from_string_literal(b"c_struct.c"),
                        29_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((p.y == 20) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"p.y == 20"),
                        Ptr::<i8>::from_string_literal(b"c_struct.c"),
                        30_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut q: Point = (p).clone();
    q.x = 99;
    {
        &(4usize);
        {
            if (((p.x == 10) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"p.x == 10"),
                        Ptr::<i8>::from_string_literal(b"c_struct.c"),
                        34_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((q.x == 99) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"q.x == 99"),
                        Ptr::<i8>::from_string_literal(b"c_struct.c"),
                        35_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((q.y == 20) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"q.y == 20"),
                        Ptr::<i8>::from_string_literal(b"c_struct.c"),
                        36_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
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
        &(4usize);
        {
            if (((l.start.x == 1) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"l.start.x == 1"),
                        Ptr::<i8>::from_string_literal(b"c_struct.c"),
                        39_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((l.end.y == 4) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"l.end.y == 4"),
                        Ptr::<i8>::from_string_literal(b"c_struct.c"),
                        40_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let a: Value<Node> = Rc::new(RefCell::new(Node {
        value: 1,
        next: Ptr::<Node>::null(),
    }));
    let mut b: Node = Node {
        value: 2,
        next: (a.as_pointer()),
    };
    {
        &(4usize);
        {
            if (((b.next.with(|__s| __s.value) == 1) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"b.next->value == 1"),
                        Ptr::<i8>::from_string_literal(b"c_struct.c"),
                        44_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
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
        &(4usize);
        {
            if (((c.inner.a == 5) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"c.inner.a == 5"),
                        Ptr::<i8>::from_string_literal(b"c_struct.c"),
                        47_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((c.inner.b == 6) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"c.inner.b == 6"),
                        Ptr::<i8>::from_string_literal(b"c_struct.c"),
                        48_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((c.color as u32) == ((Color_GREEN as i32) as u32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"c.color == GREEN"),
                        Ptr::<i8>::from_string_literal(b"c_struct.c"),
                        49_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((c.count == 42) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"c.count == 42"),
                        Ptr::<i8>::from_string_literal(b"c_struct.c"),
                        50_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut c2: Container = <Container>::default();
    c2.color = Color_BLUE;
    {
        &(4usize);
        {
            if ((((c2.color as u32) == 2_u32) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"c2.color == 2"),
                        Ptr::<i8>::from_string_literal(b"c_struct.c"),
                        54_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    return 0;
}
pub fn __assert_fail_0(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
