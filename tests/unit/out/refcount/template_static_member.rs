extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static Static_int__s_1: Value<i32> = Rc::new(RefCell::new(55));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Static_0 {}
thread_local!(
    pub static Static_char__s_3: Value<i8> = Rc::new(RefCell::new(55_i8));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Static_2 {}
thread_local!(
    pub static Static_long__s_5: Value<i64> = Rc::new(RefCell::new(55_i64));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Static_4 {}
thread_local!(
    pub static Range_int__lo_7: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!(
    pub static Range_int__hi_8: Value<i32> = Rc::new(RefCell::new(1));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Range_6 {}
thread_local!(
    pub static Range_long__lo_10: Value<i64> = Rc::new(RefCell::new((-2_i32 as i64)));
);
thread_local!(
    pub static Range_long__hi_11: Value<i64> = Rc::new(RefCell::new(2_i64));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Range_9 {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    Static_int__s_1.with(|rc| *rc.borrow_mut() = 22);
    Static_char__s_3.with(|rc| *rc.borrow_mut() = 33_i8);
    (if (Static_int__s_1.with(|rc| *rc.borrow()) == 22) {
        &(0);
    } else {
        ({
            __assert_fail_12(
                Ptr::<i8>::from_string_literal(b"Static<int>::s == 22"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((Static_char__s_3.with(|rc| *rc.borrow()) as i32) == 33) {
        &(0);
    } else {
        ({
            __assert_fail_12(
                Ptr::<i8>::from_string_literal(b"Static<char>::s == 33"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (Static_long__s_5.with(|rc| *rc.borrow()) == 55_i64) {
        &(0);
    } else {
        ({
            __assert_fail_12(
                Ptr::<i8>::from_string_literal(b"Static<long>::s == 55"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (Range_int__lo_7.with(|rc| *rc.borrow()) == -1_i32)
        && (Range_int__hi_8.with(|rc| *rc.borrow()) == 1)
    {
        &(0);
    } else {
        ({
            __assert_fail_12(
                Ptr::<i8>::from_string_literal(b"Range<int>::lo == -1 && Range<int>::hi == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (Range_long__lo_10.with(|rc| *rc.borrow()) == (-2_i32 as i64))
        && (Range_long__hi_11.with(|rc| *rc.borrow()) == 2_i64)
    {
        &(0);
    } else {
        ({
            __assert_fail_12(
                Ptr::<i8>::from_string_literal(b"Range<long>::lo == -2 && Range<long>::hi == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_12(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {
    let _ = Static_int__s_1.with(|_| ());
    let _ = Static_char__s_3.with(|_| ());
    let _ = Static_long__s_5.with(|_| ());
    let _ = Range_int__lo_7.with(|_| ());
    let _ = Range_int__hi_8.with(|_| ());
    let _ = Range_long__lo_10.with(|_| ());
    let _ = Range_long__hi_11.with(|_| ());
}
