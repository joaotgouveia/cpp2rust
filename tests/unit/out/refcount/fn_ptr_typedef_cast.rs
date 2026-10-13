extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct max_align_t {
    #[offset(0)]
    pub __clang_max_align_nonce1: i64,
    #[offset(16)]
    #[byte_size(16)]
    pub __clang_max_align_nonce2: f64,
}
impl Clone for max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub fn call_with_ulong_0(mut g: FnPtr<fn(u64) -> u64>) -> u64 {
    return ({ g.call(3_u64) }).wrapping_add(1_u64);
}
pub fn same_type_1(mut a: u64) -> u64 {
    return a;
}
pub fn via_size_t_param_2(mut b: usize) -> u64 {
    return (b as u64);
}
pub fn via_size_t_return_3(mut b: u64) -> usize {
    return (b as usize);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct pair {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
pub fn pair_scaled_4(mut p: pair, mut n: usize) -> u64 {
    return (((p.a as usize).wrapping_mul(n)).wrapping_add((p.b as usize)) as u64);
}
pub fn make_pair_5(mut n: usize) -> pair {
    let mut p: pair = pair {
        a: (n as i32),
        b: ((n as i32) * 2),
    };
    return p;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        &(4usize);
        {
            if (((({ call_with_ulong_0(FnPtr::<fn(u64) -> u64>::new(same_type_1)) }) == 4_u64)
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"call_with_ulong(same_type) == 4"),
                        Ptr::<i8>::from_string_literal(b"fn_ptr_typedef_cast.c"),
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
                call_with_ulong_0(
                    (FnPtr::<fn(usize) -> u64>::new(via_size_t_param_2)).cast::<fn(u64) -> u64>(),
                )
            }) == 4_u64) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"call_with_ulong(via_size_t_param) == 4"),
                        Ptr::<i8>::from_string_literal(b"fn_ptr_typedef_cast.c"),
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
                call_with_ulong_0(
                    FnPtr::<fn(u64) -> usize>::new(via_size_t_return_3).cast::<fn(u64) -> u64>(),
                )
            }) == 4_u64) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_6 ( Ptr::<i8 >::from_string_literal(b"call_with_ulong((unsigned long (*)(unsigned long))via_size_t_return) == 4" )  , Ptr::<i8 >::from_string_literal(b"fn_ptr_typedef_cast.c" )  , 27_u32  , Ptr::<i8 >::from_string_literal(b"int main()" )  , )
                });
            }
        }
    };
    let mut original: FnPtr<fn(usize) -> u64> = FnPtr::<fn(usize) -> u64>::new(via_size_t_param_2);
    let mut adapted: FnPtr<fn(u64) -> u64> = original.cast::<fn(u64) -> u64>();
    let mut back: FnPtr<fn(usize) -> u64> = adapted.cast::<fn(usize) -> u64>();
    {
        &(4usize);
        {
            if ((({ (back).clone() } == { (original).clone() }) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"back == original"),
                        Ptr::<i8>::from_string_literal(b"fn_ptr_typedef_cast.c"),
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
            if (((({ back.call(5_usize) }) == 5_u64) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"back(5) == 5"),
                        Ptr::<i8>::from_string_literal(b"fn_ptr_typedef_cast.c"),
                        35_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut scaled: FnPtr<fn(pair, u64) -> u64> =
        FnPtr::<fn(pair, usize) -> u64>::new(pair_scaled_4).cast::<fn(pair, u64) -> u64>();
    let mut p: pair = pair { a: 3, b: 4 };
    {
        &(4usize);
        {
            if (((({ scaled.call((p).clone(), 10_u64) }) == 34_u64) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"scaled(p, 10) == 34"),
                        Ptr::<i8>::from_string_literal(b"fn_ptr_typedef_cast.c"),
                        40_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut make: FnPtr<fn(u64) -> pair> =
        FnPtr::<fn(usize) -> pair>::new(make_pair_5).cast::<fn(u64) -> pair>();
    let mut q: pair = ({ make.call(5_u64) });
    {
        &(4usize);
        {
            if (((q.a == 5) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"q.a == 5"),
                        Ptr::<i8>::from_string_literal(b"fn_ptr_typedef_cast.c"),
                        45_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((q.b == 10) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"q.b == 10"),
                        Ptr::<i8>::from_string_literal(b"fn_ptr_typedef_cast.c"),
                        46_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    return 0;
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
