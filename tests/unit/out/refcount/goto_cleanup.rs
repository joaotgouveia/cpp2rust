extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn early_0(mut n: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    let mut intentionally_const_var: i32 = 0_i32;
    goto_block!({
        '__entry: {
            ret = 0;
            if (((n < 0) as i32) != 0) {
                ret = -1_i32;
                goto!('out);
            }
            ret = 100;
            intentionally_const_var = 22;
        }
        'out: {
            return ((ret + intentionally_const_var) - intentionally_const_var);
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn from_loop_1(mut n: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    goto_block!({
        '__entry: {
            ret = 0;
            let mut i: i32 = 0;
            'loop_: while (((i < n) as i32) != 0) {
                if (((i == 3) as i32) != 0) {
                    ret = 7;
                    goto!('out);
                }
                ret += i;
                i.postfix_inc();
            }
            ret = 999;
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn from_switch_2(mut n: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    goto_block!({
        '__entry: {
            ret = 0;
            'switch: {
                match { n } {
                    __v if __v == 1 => {
                        ret = 10;
                        goto!('out);
                    }
                    _ => {
                        ret = 20;
                        break 'switch;
                    }
                }
            };
            ret = 999;
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct wrapper {
    #[offset(0)]
    #[byte_size(8)]
    pub item: Ptr<i32>,
}
pub fn via_pointer_3(mut w: Ptr<wrapper>, mut fail: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    let mut item: Ptr<i32> = Ptr::<i32>::null();
    goto_block!({
        '__entry: {
            ret = 0;
            item = w.with(|__s| __s.item.clone());
            if (fail != 0) {
                ret = -1_i32;
                goto!('out);
            }
            ret = { (item.read()) };
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn via_arrays_4(mut fail: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    let mut remain: [u8; 4] = [0_u8; 4];
    let mut name: [i8; 5] = [0_i8; 5];
    goto_block!({
        '__entry: {
            ret = 0;
            remain = [0_u8, 0_u8, 0_u8, 0_u8];
            name = b"wxyz\0".map(i8::from_byte);
            if (fail != 0) {
                ret = -1_i32;
                goto!('out);
            }
            remain[(1) as usize] = 9_u8;
            ret = ((((remain[(0) as usize] as i32) + (remain[(1) as usize] as i32))
                + (((name[(0) as usize] as i32) == ('w' as i32)) as i32))
                + (((name[(4) as usize] as i32) == ('\0' as i32)) as i32));
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        &(4usize);
        {
            if (((({ early_0(-1_i32) }) == -1_i32) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"early(-1) == -1"),
                        Ptr::<i8>::from_string_literal(b"goto_cleanup.c"),
                        75_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ early_0(5) }) == 100) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"early(5) == 100"),
                        Ptr::<i8>::from_string_literal(b"goto_cleanup.c"),
                        76_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ from_loop_1(2) }) == 999) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"from_loop(2) == 999"),
                        Ptr::<i8>::from_string_literal(b"goto_cleanup.c"),
                        77_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ from_loop_1(10) }) == 7) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"from_loop(10) == 7"),
                        Ptr::<i8>::from_string_literal(b"goto_cleanup.c"),
                        78_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ from_switch_2(1) }) == 10) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"from_switch(1) == 10"),
                        Ptr::<i8>::from_string_literal(b"goto_cleanup.c"),
                        79_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ from_switch_2(2) }) == 999) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"from_switch(2) == 999"),
                        Ptr::<i8>::from_string_literal(b"goto_cleanup.c"),
                        80_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let value: Value<i32> = Rc::new(RefCell::new(42));
    let w: Value<wrapper> = Rc::new(RefCell::new(wrapper {
        item: (value.as_pointer()),
    }));
    {
        &(4usize);
        {
            if (((({ via_pointer_3((w.as_pointer()), 0) }) == 42) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"via_pointer(&w, 0) == 42"),
                        Ptr::<i8>::from_string_literal(b"goto_cleanup.c"),
                        83_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ via_pointer_3((w.as_pointer()), 1) }) == -1_i32) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"via_pointer(&w, 1) == -1"),
                        Ptr::<i8>::from_string_literal(b"goto_cleanup.c"),
                        84_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ via_arrays_4(0) }) == 11) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"via_arrays(0) == 11"),
                        Ptr::<i8>::from_string_literal(b"goto_cleanup.c"),
                        85_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ via_arrays_4(1) }) == -1_i32) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"via_arrays(1) == -1"),
                        Ptr::<i8>::from_string_literal(b"goto_cleanup.c"),
                        86_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    return 0;
}
pub fn __assert_fail_5(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
