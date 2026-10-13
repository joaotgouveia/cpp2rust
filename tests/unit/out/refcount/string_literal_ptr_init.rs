extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct label {
    #[offset(0)]
    #[byte_size(8)]
    pub name: Ptr<i8>,
    #[offset(8)]
    #[byte_size(8)]
    pub probe: FnPtr<fn() -> i32>,
    #[offset(16)]
    pub mask: i32,
}
pub fn probe_two_0() -> i32 {
    return 1;
}
thread_local!(
    pub static table_1: Value<Box<[label]>> = Rc::new(RefCell::new(Box::new([
        label {
            name: Ptr::<i8>::from_string_literal(b"first"),
            probe: FnPtr::<fn() -> i32>::null(),
            mask: (1 << 4),
        },
        label {
            name: Ptr::<i8>::from_string_literal(b"second"),
            probe: (FnPtr::<fn() -> i32>::new(probe_two_0)),
            mask: (1 << 5),
        },
    ])));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        {
            if (((({
                {
                    (*table_1.with(Value::clone).borrow())[(1) as usize]
                        .probe
                        .clone()
                }
                .call()
            }) == 1) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"table[1].probe() == 1"),
                        Ptr::<i8>::from_string_literal(b"string_literal_ptr_init.c"),
                        26_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        { {} }
    };
    let mut tail: Ptr<i8> = (Ptr::<i8>::from_string_literal(b"ab.cd").offset(2));
    {
        &(4usize);
        {
            if (((((elem!(tail, 0).read()) as i32) == ('.' as i32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"tail[0] == \'.\'"),
                        Ptr::<i8>::from_string_literal(b"string_literal_ptr_init.c"),
                        30_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((((elem!(tail, 1).read()) as i32) == ('c' as i32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"tail[1] == \'c\'"),
                        Ptr::<i8>::from_string_literal(b"string_literal_ptr_init.c"),
                        31_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((((elem!(tail, 2).read()) as i32) == ('d' as i32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"tail[2] == \'d\'"),
                        Ptr::<i8>::from_string_literal(b"string_literal_ptr_init.c"),
                        32_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut have: i32 = 0;
    let mut p: AnyPtr = if (have != 0) {
        ({
            (*table_1.with(Value::clone).borrow())[(0) as usize]
                .name
                .clone()
        })
        .to_any()
    } else {
        Ptr::<i8>::from_string_literal(b"").to_any()
    };
    {
        &(4usize);
        {
            if (((((elem!((p.reinterpret_cast::<i8>()), 0).read()) as i32) == ('\0' as i32))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"((const char *)p)[0] == \'\\0\'"),
                        Ptr::<i8>::from_string_literal(b"string_literal_ptr_init.c"),
                        36_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    have = 1;
    p = if (have != 0) {
        ({
            (*table_1.with(Value::clone).borrow())[(0) as usize]
                .name
                .clone()
        })
        .to_any()
    } else {
        Ptr::<i8>::from_string_literal(b"").to_any()
    };
    {
        &(4usize);
        {
            if (((((elem!((p.reinterpret_cast::<i8>()), 0).read()) as i32) == ('f' as i32)) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"((const char *)p)[0] == \'f\'"),
                        Ptr::<i8>::from_string_literal(b"string_literal_ptr_init.c"),
                        39_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    return 0;
}
pub fn __assert_fail_2(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {
    let _ = table_1.with(|_| ());
}
