extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static next_0: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn bump_1(mut v: Ptr<std_vector_int__std_allocator_int__>, mut x: i32) -> i32 {
    {
        let __rhs = x;
        elem!((v), 0_u64).with_mut(|__v| *__v = *__v + __rhs)
    };
    return (*next_0.with(Value::clone).borrow_mut()).postfix_inc();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new({
        let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
        std_vector_int__std_allocator_int__::new_1({ 4_u64 }, { __tmp_0.as_pointer() }, None)
    }));
    let items: Value<std_vector_unsigned_int__std_allocator_unsigned_int__> = Rc::new(
        RefCell::new(std_vector_unsigned_int__std_allocator_unsigned_int__::new_1({ 3_u64 }, None)),
    );
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < ({ (*items.borrow()).size() })) {
        let __rhs = (i).wrapping_mul(2_u32);
        (*items.borrow_mut())[(i as u64)] = __rhs;
        i.prefix_inc();
    }
    {
        let rhs_0 = ({ (*items.borrow())[1_u64] }).wrapping_add(5_u32);
        (*items.borrow_mut())[1_u64] = rhs_0
    };
    (*items.borrow_mut())[2_u64].postfix_inc();
    ({
        cpp2rust_assert_fail_2(
            (({ (*items.borrow())[0_u64] } == 0_u32) && ({ (*items.borrow())[1_u64] } == 7_u32))
                && ({ (*items.borrow())[2_u64] } == 5_u32),
        )
    });
    elem!(v.as_pointer(), ({ (*v.borrow())[0_u64] } as u64)).write(3);
    let __rhs = ({ (*v.borrow())[1_u64] } + 4);
    elem!(v.as_pointer(), ({ (*v.borrow())[1_u64] } as u64)).write(__rhs);
    ({
        cpp2rust_assert_fail_2(({ (*v.borrow())[1_u64] } == 3) && ({ (*v.borrow())[3_u64] } == 7))
    });
    let mut p: Ptr<i32> = (v.as_pointer().offset(0_u64));
    elem!(v.as_pointer(), ((p.read()) as u64)).write(9);
    ({ cpp2rust_assert_fail_2(({ (*v.borrow())[1_u64] } == 9)) });
    elem!(v.as_pointer(), (({ bump_1((v.as_pointer()), 10,) }) as u64)).write(2);
    ({ cpp2rust_assert_fail_2(({ (*v.borrow())[0_u64] } == 2)) });
    ({
        cpp2rust_assert_fail_2(
            (({
                let _v: Ptr<std_vector_int__std_allocator_int__> = (v.as_pointer());
                let _x: i32 = { (*v.borrow())[2_u64] };
                bump_1(_v, _x)
            }) == 1)
                && ({ (*v.borrow())[0_u64] } == 3),
        )
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = next_0.with(|_| ());
}
