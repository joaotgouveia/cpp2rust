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
pub type std_byte = u8;
pub fn my_foo_0(mut p: AnyPtr) -> i32 {
    return (p.reinterpret_cast::<i32>().read());
}
pub fn foo_1(mut fn_: FnPtr<fn(AnyPtr) -> i32>, mut pi: Ptr<i32>) -> i32 {
    return ({ fn_.call((pi as Ptr<i32>).to_any()) });
}
pub fn twice_2(mut x: u64) -> u64 {
    return (x).wrapping_mul(2_u64);
}
pub fn twice_in_place_3(x: Ptr<u64>) -> u64 {
    x.write({ (x.read()).wrapping_mul(2_u64) });
    return (x.read());
}
pub fn ret_size_4(mut v: i32) -> usize {
    return ((v + 1) as usize);
}
pub fn call_fn_5(mut f: FnPtr<fn(i32) -> usize>, mut v: i32) -> usize {
    return ({ f.call(v) }).wrapping_mul(2_usize);
}
pub fn identity_hash_6(mut v: bool) -> usize {
    return (v as usize);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct HashHolder_7 {
    #[offset(0)]
    #[byte_size(8)]
    pub h: FnPtr<fn(bool) -> u64>,
}
impl HashHolder_7 {
    pub fn new(h: Ptr<FnPtr<fn(bool) -> u64>>) -> Self {
        Self { h: (h.read()) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct HashHolder_8 {
    #[offset(0)]
    #[byte_size(8)]
    pub h: FnPtr<fn(u64) -> u64>,
}
impl HashHolder_8 {
    pub fn new(h: Ptr<FnPtr<fn(u64) -> u64>>) -> Self {
        Self { h: (h.read()) }
    }
}
pub fn hash_identity_9(mut v: u64) -> usize {
    return (v as usize);
}
pub fn test_hash_10(mut v: u64) {
    let mut hh: HashHolder_8 = {
        let __tmp_0: Value<FnPtr<fn(u64) -> u64>> = Rc::new(RefCell::new(
            (FnPtr::<fn(u64) -> usize>::new(hash_identity_9)).cast::<fn(u64) -> u64>(),
        ));
        HashHolder_8::new({ __tmp_0.as_pointer() })
    };
    (if (({ hh.h.call(v) }) == 7_u64) {
        &(0);
    } else {
        ({
            __assert_fail_11(
                Ptr::<i8>::from_string_literal(b"hh.h(v) == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_hash(T) [T = unsigned long]"),
            )
        });
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut fn_: FnPtr<fn(AnyPtr) -> i32> = FnPtr::<fn(AnyPtr) -> i32>::null();
    (if (fn_).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_11(
                Ptr::<i8>::from_string_literal(b"fn == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (fn_).clone() } != { FnPtr::<fn(AnyPtr) -> i32>::new(my_foo_0) }) {
        &(0);
    } else {
        ({
            __assert_fail_11(
                Ptr::<i8>::from_string_literal(b"fn != my_foo"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    fn_ = FnPtr::<fn(AnyPtr) -> i32>::new(my_foo_0);
    (if !((fn_).is_null()) {
        &(0);
    } else {
        ({
            __assert_fail_11(
                Ptr::<i8>::from_string_literal(b"fn != nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (fn_).clone() } == { FnPtr::<fn(AnyPtr) -> i32>::new(my_foo_0) }) {
        &(0);
    } else {
        ({
            __assert_fail_11(
                Ptr::<i8>::from_string_literal(b"fn == my_foo"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let a: Value<i32> = Rc::new(RefCell::new(10));
    (if ({ ({ foo_1((fn_).clone(), (a.as_pointer())) }) } == { (*a.borrow()) }) {
        &(0);
    } else {
        ({
            __assert_fail_11(
                Ptr::<i8>::from_string_literal(b"foo(fn, &a) == a"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut ul_fn: FnPtr<fn(u64) -> u64> = (FnPtr::<fn(u64) -> u64>::new(twice_2));
    let mut n: usize = 21_usize;
    let mut r: usize = (({ ul_fn.call((n as u64)) }) as usize);
    (if (r == 42_usize) {
        &(0);
    } else {
        ({
            __assert_fail_11(
                Ptr::<i8>::from_string_literal(b"r == 42"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut ul_ref_fn: FnPtr<fn(Ptr<u64>) -> u64> =
        (FnPtr::<fn(Ptr<u64>) -> u64>::new(twice_in_place_3));
    let m: Value<usize> = Rc::new(RefCell::new(21_usize));
    let mut q: usize = (({ ul_ref_fn.call((m.as_pointer()).reinterpret_cast::<u64>()) }) as usize);
    (if (q == 42_usize) {
        &(0);
    } else {
        ({
            __assert_fail_11(
                Ptr::<i8>::from_string_literal(b"q == 42"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((*m.borrow()) == 42_usize) {
        &(0);
    } else {
        ({
            __assert_fail_11(
                Ptr::<i8>::from_string_literal(b"m == 42"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ call_fn_5(FnPtr::<fn(i32) -> usize>::new(ret_size_4), 3) }) == 8_usize) {
        &(0);
    } else {
        ({
            __assert_fail_11(
                Ptr::<i8>::from_string_literal(b"call_fn(ret_size<int>, 3) == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut hh: HashHolder_7 = {
        let __tmp_1: Value<FnPtr<fn(bool) -> u64>> = Rc::new(RefCell::new(
            (FnPtr::<fn(bool) -> usize>::new(identity_hash_6)).cast::<fn(bool) -> u64>(),
        ));
        HashHolder_7::new({ __tmp_1.as_pointer() })
    };
    (if (({ hh.h.call(true) }) == 1_u64) {
        &(0);
    } else {
        ({
            __assert_fail_11(
                Ptr::<i8>::from_string_literal(b"hh.h(true) == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ test_hash_10(7_u64) });
    return 0;
}
pub fn __assert_fail_11(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
