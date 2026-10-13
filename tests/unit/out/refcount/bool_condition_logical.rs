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
pub type Code = u32;
pub const Code_CODE_OK: Code = 0;
pub const Code_CODE_ERR: Code = 1;
pub const Code_CODE_FATAL: Code = 2;
thread_local!(
    pub static side_effect_0: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn observe_1(mut v: i32) -> i32 {
    (*side_effect_0.with(Value::clone).borrow_mut()).prefix_inc();
    return v;
}
pub fn returns_one_2() -> i32 {
    return 1;
}
pub fn returns_zero_3() -> i32 {
    return 0;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut n: i32 = 3;
    let mut zero: i32 = 0;
    let storage: Value<i32> = Rc::new(RefCell::new(7));
    let mut p: Ptr<i32> = (storage.as_pointer());
    let mut np: Ptr<i32> = Ptr::<i32>::null();
    let mut u: u32 = 4_u32;
    let mut code: Code = Code_CODE_OK;
    if (n != 0) && (!(p).is_null()) {
        (if true {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"true"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    if (n != 0) && (!(np).is_null()) {
        (if false {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"false"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    if (zero != 0) || (!(p).is_null()) {
        (if true {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"true"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    if (zero != 0) || (!(np).is_null()) {
        (if false {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"false"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    if (((n != 0) && (u != 0)) && (!(p).is_null())) && ((code as i32) == (Code_CODE_OK as i32)) {
        (if true {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"true"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    side_effect_0.with(|rc| *rc.borrow_mut() = 0);
    if (zero != 0) && (({ observe_1(1) }) != 0) {
        (if false {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"false"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    (if (side_effect_0.with(|rc| *rc.borrow()) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"side_effect == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    if (n != 0) || (({ observe_1(1) }) != 0) {
        (if true {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"true"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    (if (side_effect_0.with(|rc| *rc.borrow()) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"side_effect == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut x: i32 = 5;
    let mut y: i32 = 3;
    let mut flags: u32 = 2_u32;
    if (x > y) || ((flags & 1_u32) != 0) {
        (if true {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"true"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    if (x < y) || ((flags & 1_u32) != 0) {
        (if false {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"false"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    let mut a: u32 = 1_u32;
    let mut b: u32 = 2_u32;
    let mut c: u32 = 3_u32;
    if (a != c) && (b != c) {
        (if true {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"true"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    let mut s: i32 = -1_i32;
    if (!((p).is_null())) && (s < 0) {
        (if true {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"true"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    let mut k: u32 = 2_u32;
    let mut done: bool = false;
    if (k > 1_u32) || (!(done)) {
        (if true {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"true"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    if (x > y) || ((flags & 4_u32) != 0) {
        (if true {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"true"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    let mut ull: u64 = 7_u64;
    if (!((p).is_null())) && (ull != 0) {
        (if true {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"true"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    if (x > y) && (ull != 0) {
        (if true {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"true"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    let mut mask: i64 = ((1_i64 << 4) | (1_i64 << 5));
    let mut bits: i64 = (1_i64 << 4);
    if (n != 0) && ((bits & mask) != 0) {
        (if true {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"true"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    if (n != 0) || ((bits & 256_i64) != 0) {
        (if true {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"true"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    let mut cp: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hi");
    let mut cnp: Ptr<i8> = Ptr::<i8>::null();
    if (x > y) && (!(cp).is_null()) {
        (if true {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"true"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    if (x < y) || (!(cnp).is_null()) {
        (if false {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"false"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    if (x > y) && ((n != 0) && (!(cp).is_null())) {
        (if true {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"true"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    if (x > y) && (({ returns_one_2() }) != 0) {
        (if true {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"true"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    if (x > y) && (!(({ returns_zero_3() }) != 0)) {
        (if true {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"true"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    if (x < y) || (({ returns_one_2() }) != 0) {
        (if true {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"true"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    if (x < y) || (!(({ returns_one_2() }) != 0)) {
        (if false {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"false"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    if ((!((p).is_null())) && (({ returns_one_2() }) != 0)) && (n != 0) {
        (if true {
            &(0);
        } else {
            ({
                __assert_fail_4(
                    Ptr::<i8>::from_string_literal(b"true"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    return 0;
}
pub fn __assert_fail_4(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {
    let _ = side_effect_0.with(|_| ());
}
