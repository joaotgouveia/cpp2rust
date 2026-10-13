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
pub struct Named {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    pub c: i32,
    #[offset(4)]
    pub d: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct anon_1 {
    #[offset(0)]
    pub g: i32,
    #[offset(4)]
    pub h: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct anon_2 {
    #[offset(0)]
    pub e: i32,
    #[offset(4)]
    pub f: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct anon_4 {
    #[offset(0)]
    pub j: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct anon_5 {
    #[offset(0)]
    pub k: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct anon_3 {
    #[offset(0)]
    pub i: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub inner_named: anon_4,
    #[offset(8)]
    #[byte_size(4)]
    pub anon_5: anon_5,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(44)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(8)]
    pub named: Named,
    #[offset(8)]
    #[byte_size(8)]
    pub anon0: anon_0,
    #[offset(16)]
    #[byte_size(8)]
    pub anon1: anon_1,
    #[offset(24)]
    #[byte_size(8)]
    pub anon_2: anon_2,
    #[offset(32)]
    #[byte_size(12)]
    pub anon_3: anon_3,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
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
        &(4usize);
        {
            if (((o.named.a == 1) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"o.named.a == 1"),
                        Ptr::<i8>::from_string_literal(b"anonymous-struct_c.c"),
                        50_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((o.named.b == 2) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"o.named.b == 2"),
                        Ptr::<i8>::from_string_literal(b"anonymous-struct_c.c"),
                        51_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((o.anon0.c == 3) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"o.anon0.c == 3"),
                        Ptr::<i8>::from_string_literal(b"anonymous-struct_c.c"),
                        52_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((o.anon0.d == 4) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"o.anon0.d == 4"),
                        Ptr::<i8>::from_string_literal(b"anonymous-struct_c.c"),
                        53_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((o.anon1.g == 5) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"o.anon1.g == 5"),
                        Ptr::<i8>::from_string_literal(b"anonymous-struct_c.c"),
                        54_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((o.anon1.h == 6) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"o.anon1.h == 6"),
                        Ptr::<i8>::from_string_literal(b"anonymous-struct_c.c"),
                        55_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((o.anon_2.e == 7) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"o.e == 7"),
                        Ptr::<i8>::from_string_literal(b"anonymous-struct_c.c"),
                        56_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((o.anon_2.f == 8) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"o.f == 8"),
                        Ptr::<i8>::from_string_literal(b"anonymous-struct_c.c"),
                        57_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((o.anon_3.i == 9) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"o.i == 9"),
                        Ptr::<i8>::from_string_literal(b"anonymous-struct_c.c"),
                        58_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((o.anon_3.inner_named.j == 10) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"o.inner_named.j == 10"),
                        Ptr::<i8>::from_string_literal(b"anonymous-struct_c.c"),
                        59_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((o.anon_3.anon_5.k == 11) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"o.k == 11"),
                        Ptr::<i8>::from_string_literal(b"anonymous-struct_c.c"),
                        60_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let mut s: anon_7 = <anon_7>::default();
    s.x = 1;
    s.z = 2;
    {
        &(4usize);
        {
            if ({
                s.x = 1;
                s.x
            } != 0)
            {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"s.x = 1"),
                        Ptr::<i8>::from_string_literal(b"anonymous-struct_c.c"),
                        70_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ({
                s.z = 2;
                s.z
            } != 0)
            {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"s.z = 2"),
                        Ptr::<i8>::from_string_literal(b"anonymous-struct_c.c"),
                        71_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    return 0;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct anon_7 {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub z: i32,
}
pub fn __assert_fail_6(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
