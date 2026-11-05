extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Degrees {
    #[offset(0)]
    pub value: i32,
}
impl Degrees {
    pub fn new(mut v: i32) -> Self {
        Self { value: v }
    }
}
impl CppMove for Degrees {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    let output: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..3).map(|_| 0_i32).collect::<Box<[i32]>>()));
    ({
        let ___first: Ptr<i32> = (input.as_pointer() as Ptr<i32>);
        let ___last: Ptr<i32> = (input.as_pointer() as Ptr<i32>).offset((3) as isize);
        copy_0(___first, ___last, (output.as_pointer() as Ptr<i32>))
    });
    let widened: Value<std_vector_long__std_allocator_long__> = Rc::new(RefCell::new({
        let __tmp_0: Value<i64> = Rc::new(RefCell::new(0_i64));
        std_vector_long__std_allocator_long__::new_1({ 3_u64 }, { __tmp_0.as_pointer() }, None)
    }));
    ({
        let ___first: Ptr<i32> = (input.as_pointer() as Ptr<i32>);
        let ___last: Ptr<i32> = (input.as_pointer() as Ptr<i32>).offset((3) as isize);
        copy_1(___first, ___last, ({ (*widened.borrow()).begin_28() }))
    });
    ({
        cpp2rust_assert_fail_2(
            ({ (*widened.borrow())[0_u64] } == 1_i64) && ({ (*widened.borrow())[2_u64] } == 3_i64),
        )
    });
    let constructed: Value<std_vector_Degrees__std_allocator_Degrees__> = Rc::new(RefCell::new({
        let __tmp_1: Value<Degrees> = Rc::new(RefCell::new(Degrees::new({ 0 })));
        std_vector_Degrees__std_allocator_Degrees__::new_1(
            { 3_u64 },
            { __tmp_1.as_pointer() },
            None,
        )
    }));
    ({
        let ___first: Ptr<i32> = (input.as_pointer() as Ptr<i32>);
        let ___last: Ptr<i32> = (input.as_pointer() as Ptr<i32>).offset((3) as isize);
        copy_3(___first, ___last, ({ (*constructed.borrow()).begin_28() }))
    });
    ({
        cpp2rust_assert_fail_2(
            ({ (*constructed.borrow())[0_u64].value } == 1)
                && ({ (*constructed.borrow())[2_u64].value } == 3),
        )
    });
    let wide: Value<Box<[i64]>> = Rc::new(RefCell::new(Box::new([7_i64, 8_i64])));
    let floating: Value<std_vector_double__std_allocator_double__> = Rc::new(RefCell::new({
        let __tmp_2: Value<f64> = Rc::new(RefCell::new(0_f64));
        std_vector_double__std_allocator_double__::new_1({ 2_u64 }, { __tmp_2.as_pointer() }, None)
    }));
    ({
        let ___first: Ptr<i64> = (wide.as_pointer() as Ptr<i64>);
        let ___last: Ptr<i64> = (wide.as_pointer() as Ptr<i64>).offset((2) as isize);
        copy_4(___first, ___last, ({ (*floating.borrow()).begin_28() }))
    });
    ({ cpp2rust_assert_fail_2(({ (*floating.borrow())[1_u64] } == 8_f64)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
