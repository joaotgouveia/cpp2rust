extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> u64 {
        ({
            cpp2rust_assert_fail_4(
                (alloc_count_1.with(|rc| *rc.borrow()) >= dealloc_count_2.with(|rc| *rc.borrow())),
            )
        });
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        ({ cpp2rust_assert_fail_4((({ malloc_allocator_base::outstanding_alloc() }) == 0_u64)) });
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_u64);
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_u64);
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_u64);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static alloc_count_1: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static dealloc_count_2: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static disable_default_constructor_3: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn test_5() -> bool {
    {
        let ar: Value<Box<[std_pair_int__double_]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
                let __tmp_1: Value<f64> = Rc::new(RefCell::new(1.5_f64));
                std_pair_int__double_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
            },
            {
                let __tmp_2: Value<i32> = Rc::new(RefCell::new(2));
                let __tmp_3: Value<f64> = Rc::new(RefCell::new(2.5_f64));
                std_pair_int__double_::new_1({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
            },
            {
                let __tmp_4: Value<i32> = Rc::new(RefCell::new(3));
                let __tmp_5: Value<f64> = Rc::new(RefCell::new(3.5_f64));
                std_pair_int__double_::new_1({ __tmp_4.as_pointer() }, { __tmp_5.as_pointer() })
            },
            {
                let __tmp_6: Value<i32> = Rc::new(RefCell::new(4));
                let __tmp_7: Value<f64> = Rc::new(RefCell::new(4.5_f64));
                std_pair_int__double_::new_1({ __tmp_6.as_pointer() }, { __tmp_7.as_pointer() })
            },
            {
                let __tmp_8: Value<i32> = Rc::new(RefCell::new(5));
                let __tmp_9: Value<f64> = Rc::new(RefCell::new(5.5_f64));
                std_pair_int__double_::new_1({ __tmp_8.as_pointer() }, { __tmp_9.as_pointer() })
            },
            {
                let __tmp_10: Value<i32> = Rc::new(RefCell::new(6));
                let __tmp_11: Value<f64> = Rc::new(RefCell::new(6.5_f64));
                std_pair_int__double_::new_1({ __tmp_10.as_pointer() }, { __tmp_11.as_pointer() })
            },
            {
                let __tmp_12: Value<i32> = Rc::new(RefCell::new(7));
                let __tmp_13: Value<f64> = Rc::new(RefCell::new(7.5_f64));
                std_pair_int__double_::new_1({ __tmp_12.as_pointer() }, { __tmp_13.as_pointer() })
            },
            {
                let __tmp_14: Value<i32> = Rc::new(RefCell::new(8));
                let __tmp_15: Value<f64> = Rc::new(RefCell::new(8.5_f64));
                std_pair_int__double_::new_1({ __tmp_14.as_pointer() }, { __tmp_15.as_pointer() })
            },
        ])));
        let m: Value<
            std_map_int__double__std_less_int___std_allocator_std_pair_const_int__double___,
        > = Rc::new(RefCell::new(
            std_map_int__double__std_less_int___std_allocator_std_pair_const_int__double___::new_1(
                { (ar.as_pointer() as Ptr<std_pair_int__double_>) },
                {
                    (ar.as_pointer() as Ptr<std_pair_int__double_>)
                        .offset(((128usize as u64).wrapping_div((16usize as u64))) as isize)
                },
            ),
        ));
        ({ cpp2rust_assert_fail_4((({ (*m.borrow()).size() }) == 8_u64)) });
        let mut s: u64 = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(9));
            (*m.borrow()).erase_68(___x.as_pointer())
        });
        ({ cpp2rust_assert_fail_4((s == 0_u64)) });
        ({ cpp2rust_assert_fail_4((({ (*m.borrow()).size() }) == 8_u64)) });
        ({ cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).first } == 1)) });
        ({
            cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).second } == 1.5_f64))
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), None) }).read()).first } == 2),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), None) }).read()).second } == 2.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).first } == 3),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).second }
                    == 3.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(3_i64)) }).read()).first } == 4),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(3_i64)) }).read()).second }
                    == 4.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(4_i64)) }).read()).first } == 5),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(4_i64)) }).read()).second }
                    == 5.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(5_i64)) }).read()).first } == 6),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(5_i64)) }).read()).second }
                    == 6.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(6_i64)) }).read()).first } == 7),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(6_i64)) }).read()).second }
                    == 7.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(7_i64)) }).read()).first } == 8),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(7_i64)) }).read()).second }
                    == 8.5_f64),
            )
        });
        s = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(4));
            (*m.borrow()).erase_68(___x.as_pointer())
        });
        ({ cpp2rust_assert_fail_4((({ (*m.borrow()).size() }) == 7_u64)) });
        ({ cpp2rust_assert_fail_4((s == 1_u64)) });
        ({ cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).first } == 1)) });
        ({
            cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).second } == 1.5_f64))
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), None) }).read()).first } == 2),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), None) }).read()).second } == 2.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).first } == 3),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).second }
                    == 3.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(3_i64)) }).read()).first } == 5),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(3_i64)) }).read()).second }
                    == 5.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(4_i64)) }).read()).first } == 6),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(4_i64)) }).read()).second }
                    == 6.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(5_i64)) }).read()).first } == 7),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(5_i64)) }).read()).second }
                    == 7.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(6_i64)) }).read()).first } == 8),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(6_i64)) }).read()).second }
                    == 8.5_f64),
            )
        });
        s = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(1));
            (*m.borrow()).erase_68(___x.as_pointer())
        });
        ({ cpp2rust_assert_fail_4((({ (*m.borrow()).size() }) == 6_u64)) });
        ({ cpp2rust_assert_fail_4((s == 1_u64)) });
        ({ cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).first } == 2)) });
        ({
            cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).second } == 2.5_f64))
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), None) }).read()).first } == 3),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), None) }).read()).second } == 3.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).first } == 5),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).second }
                    == 5.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(3_i64)) }).read()).first } == 6),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(3_i64)) }).read()).second }
                    == 6.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(4_i64)) }).read()).first } == 7),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(4_i64)) }).read()).second }
                    == 7.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(5_i64)) }).read()).first } == 8),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(5_i64)) }).read()).second }
                    == 8.5_f64),
            )
        });
        s = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(8));
            (*m.borrow()).erase_68(___x.as_pointer())
        });
        ({ cpp2rust_assert_fail_4((({ (*m.borrow()).size() }) == 5_u64)) });
        ({ cpp2rust_assert_fail_4((s == 1_u64)) });
        ({ cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).first } == 2)) });
        ({
            cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).second } == 2.5_f64))
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), None) }).read()).first } == 3),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), None) }).read()).second } == 3.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).first } == 5),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).second }
                    == 5.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(3_i64)) }).read()).first } == 6),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(3_i64)) }).read()).second }
                    == 6.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(4_i64)) }).read()).first } == 7),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(4_i64)) }).read()).second }
                    == 7.5_f64),
            )
        });
        s = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(3));
            (*m.borrow()).erase_68(___x.as_pointer())
        });
        ({ cpp2rust_assert_fail_4((({ (*m.borrow()).size() }) == 4_u64)) });
        ({ cpp2rust_assert_fail_4((s == 1_u64)) });
        ({ cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).first } == 2)) });
        ({
            cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).second } == 2.5_f64))
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), None) }).read()).first } == 5),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), None) }).read()).second } == 5.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).first } == 6),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).second }
                    == 6.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(3_i64)) }).read()).first } == 7),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(3_i64)) }).read()).second }
                    == 7.5_f64),
            )
        });
        s = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(6));
            (*m.borrow()).erase_68(___x.as_pointer())
        });
        ({ cpp2rust_assert_fail_4((({ (*m.borrow()).size() }) == 3_u64)) });
        ({ cpp2rust_assert_fail_4((s == 1_u64)) });
        ({ cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).first } == 2)) });
        ({
            cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).second } == 2.5_f64))
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), None) }).read()).first } == 5),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), None) }).read()).second } == 5.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).first } == 7),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).second }
                    == 7.5_f64),
            )
        });
        s = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(7));
            (*m.borrow()).erase_68(___x.as_pointer())
        });
        ({ cpp2rust_assert_fail_4((({ (*m.borrow()).size() }) == 2_u64)) });
        ({ cpp2rust_assert_fail_4((s == 1_u64)) });
        ({ cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).first } == 2)) });
        ({
            cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).second } == 2.5_f64))
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), None) }).read()).first } == 5),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_6(({ (*m.borrow()).begin_20() }), None) }).read()).second } == 5.5_f64),
            )
        });
        s = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(2));
            (*m.borrow()).erase_68(___x.as_pointer())
        });
        ({ cpp2rust_assert_fail_4((({ (*m.borrow()).size() }) == 1_u64)) });
        ({ cpp2rust_assert_fail_4((s == 1_u64)) });
        ({ cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).first } == 5)) });
        ({
            cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).second } == 5.5_f64))
        });
        s = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(5));
            (*m.borrow()).erase_68(___x.as_pointer())
        });
        ({ cpp2rust_assert_fail_4((({ (*m.borrow()).size() }) == 0_u64)) });
        ({ cpp2rust_assert_fail_4((s == 1_u64)) });
    }
    {
        let ar: Value<Box<[std_pair_int__double_]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_16: Value<i32> = Rc::new(RefCell::new(1));
                let __tmp_17: Value<f64> = Rc::new(RefCell::new(1.5_f64));
                std_pair_int__double_::new_1({ __tmp_16.as_pointer() }, { __tmp_17.as_pointer() })
            },
            {
                let __tmp_18: Value<i32> = Rc::new(RefCell::new(2));
                let __tmp_19: Value<f64> = Rc::new(RefCell::new(2.5_f64));
                std_pair_int__double_::new_1({ __tmp_18.as_pointer() }, { __tmp_19.as_pointer() })
            },
            {
                let __tmp_20: Value<i32> = Rc::new(RefCell::new(3));
                let __tmp_21: Value<f64> = Rc::new(RefCell::new(3.5_f64));
                std_pair_int__double_::new_1({ __tmp_20.as_pointer() }, { __tmp_21.as_pointer() })
            },
            {
                let __tmp_22: Value<i32> = Rc::new(RefCell::new(4));
                let __tmp_23: Value<f64> = Rc::new(RefCell::new(4.5_f64));
                std_pair_int__double_::new_1({ __tmp_22.as_pointer() }, { __tmp_23.as_pointer() })
            },
            {
                let __tmp_24: Value<i32> = Rc::new(RefCell::new(5));
                let __tmp_25: Value<f64> = Rc::new(RefCell::new(5.5_f64));
                std_pair_int__double_::new_1({ __tmp_24.as_pointer() }, { __tmp_25.as_pointer() })
            },
            {
                let __tmp_26: Value<i32> = Rc::new(RefCell::new(6));
                let __tmp_27: Value<f64> = Rc::new(RefCell::new(6.5_f64));
                std_pair_int__double_::new_1({ __tmp_26.as_pointer() }, { __tmp_27.as_pointer() })
            },
            {
                let __tmp_28: Value<i32> = Rc::new(RefCell::new(7));
                let __tmp_29: Value<f64> = Rc::new(RefCell::new(7.5_f64));
                std_pair_int__double_::new_1({ __tmp_28.as_pointer() }, { __tmp_29.as_pointer() })
            },
            {
                let __tmp_30: Value<i32> = Rc::new(RefCell::new(8));
                let __tmp_31: Value<f64> = Rc::new(RefCell::new(8.5_f64));
                std_pair_int__double_::new_1({ __tmp_30.as_pointer() }, { __tmp_31.as_pointer() })
            },
        ])));
        let m: Value<
            std_map_int__double__std_less_int___min_allocator_std_pair_const_int__double___,
        > = Rc::new(RefCell::new(
            std_map_int__double__std_less_int___min_allocator_std_pair_const_int__double___::new_1(
                { (ar.as_pointer() as Ptr<std_pair_int__double_>) },
                {
                    (ar.as_pointer() as Ptr<std_pair_int__double_>)
                        .offset(((128usize as u64).wrapping_div((16usize as u64))) as isize)
                },
            ),
        ));
        ({ cpp2rust_assert_fail_4((({ (*m.borrow()).size() }) == 8_u64)) });
        let mut s: u64 = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(9));
            (*m.borrow()).erase_68(___x.as_pointer())
        });
        ({ cpp2rust_assert_fail_4((s == 0_u64)) });
        ({ cpp2rust_assert_fail_4((({ (*m.borrow()).size() }) == 8_u64)) });
        ({ cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).first } == 1)) });
        ({
            cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).second } == 1.5_f64))
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), None) }).read()).first } == 2),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), None) }).read()).second } == 2.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).first } == 3),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).second }
                    == 3.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(3_i64)) }).read()).first } == 4),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(3_i64)) }).read()).second }
                    == 4.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(4_i64)) }).read()).first } == 5),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(4_i64)) }).read()).second }
                    == 5.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(5_i64)) }).read()).first } == 6),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(5_i64)) }).read()).second }
                    == 6.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(6_i64)) }).read()).first } == 7),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(6_i64)) }).read()).second }
                    == 7.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(7_i64)) }).read()).first } == 8),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(7_i64)) }).read()).second }
                    == 8.5_f64),
            )
        });
        s = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(4));
            (*m.borrow()).erase_68(___x.as_pointer())
        });
        ({ cpp2rust_assert_fail_4((({ (*m.borrow()).size() }) == 7_u64)) });
        ({ cpp2rust_assert_fail_4((s == 1_u64)) });
        ({ cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).first } == 1)) });
        ({
            cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).second } == 1.5_f64))
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), None) }).read()).first } == 2),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), None) }).read()).second } == 2.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).first } == 3),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).second }
                    == 3.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(3_i64)) }).read()).first } == 5),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(3_i64)) }).read()).second }
                    == 5.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(4_i64)) }).read()).first } == 6),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(4_i64)) }).read()).second }
                    == 6.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(5_i64)) }).read()).first } == 7),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(5_i64)) }).read()).second }
                    == 7.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(6_i64)) }).read()).first } == 8),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(6_i64)) }).read()).second }
                    == 8.5_f64),
            )
        });
        s = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(1));
            (*m.borrow()).erase_68(___x.as_pointer())
        });
        ({ cpp2rust_assert_fail_4((({ (*m.borrow()).size() }) == 6_u64)) });
        ({ cpp2rust_assert_fail_4((s == 1_u64)) });
        ({ cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).first } == 2)) });
        ({
            cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).second } == 2.5_f64))
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), None) }).read()).first } == 3),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), None) }).read()).second } == 3.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).first } == 5),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).second }
                    == 5.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(3_i64)) }).read()).first } == 6),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(3_i64)) }).read()).second }
                    == 6.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(4_i64)) }).read()).first } == 7),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(4_i64)) }).read()).second }
                    == 7.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(5_i64)) }).read()).first } == 8),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(5_i64)) }).read()).second }
                    == 8.5_f64),
            )
        });
        s = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(8));
            (*m.borrow()).erase_68(___x.as_pointer())
        });
        ({ cpp2rust_assert_fail_4((({ (*m.borrow()).size() }) == 5_u64)) });
        ({ cpp2rust_assert_fail_4((s == 1_u64)) });
        ({ cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).first } == 2)) });
        ({
            cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).second } == 2.5_f64))
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), None) }).read()).first } == 3),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), None) }).read()).second } == 3.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).first } == 5),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).second }
                    == 5.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(3_i64)) }).read()).first } == 6),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(3_i64)) }).read()).second }
                    == 6.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(4_i64)) }).read()).first } == 7),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(4_i64)) }).read()).second }
                    == 7.5_f64),
            )
        });
        s = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(3));
            (*m.borrow()).erase_68(___x.as_pointer())
        });
        ({ cpp2rust_assert_fail_4((({ (*m.borrow()).size() }) == 4_u64)) });
        ({ cpp2rust_assert_fail_4((s == 1_u64)) });
        ({ cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).first } == 2)) });
        ({
            cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).second } == 2.5_f64))
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), None) }).read()).first } == 5),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), None) }).read()).second } == 5.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).first } == 6),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).second }
                    == 6.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(3_i64)) }).read()).first } == 7),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(3_i64)) }).read()).second }
                    == 7.5_f64),
            )
        });
        s = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(6));
            (*m.borrow()).erase_68(___x.as_pointer())
        });
        ({ cpp2rust_assert_fail_4((({ (*m.borrow()).size() }) == 3_u64)) });
        ({ cpp2rust_assert_fail_4((s == 1_u64)) });
        ({ cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).first } == 2)) });
        ({
            cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).second } == 2.5_f64))
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), None) }).read()).first } == 5),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), None) }).read()).second } == 5.5_f64),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).first } == 7),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), Some(2_i64)) }).read()).second }
                    == 7.5_f64),
            )
        });
        s = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(7));
            (*m.borrow()).erase_68(___x.as_pointer())
        });
        ({ cpp2rust_assert_fail_4((({ (*m.borrow()).size() }) == 2_u64)) });
        ({ cpp2rust_assert_fail_4((s == 1_u64)) });
        ({ cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).first } == 2)) });
        ({
            cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).second } == 2.5_f64))
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), None) }).read()).first } == 5),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ({ (({ next_7(({ (*m.borrow()).begin_20() }), None) }).read()).second } == 5.5_f64),
            )
        });
        s = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(2));
            (*m.borrow()).erase_68(___x.as_pointer())
        });
        ({ cpp2rust_assert_fail_4((({ (*m.borrow()).size() }) == 1_u64)) });
        ({ cpp2rust_assert_fail_4((s == 1_u64)) });
        ({ cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).first } == 5)) });
        ({
            cpp2rust_assert_fail_4(({ (({ (*m.borrow()).begin_20() }).read()).second } == 5.5_f64))
        });
        s = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(5));
            (*m.borrow()).erase_68(___x.as_pointer())
        });
        ({ cpp2rust_assert_fail_4((({ (*m.borrow()).size() }) == 0_u64)) });
        ({ cpp2rust_assert_fail_4((s == 1_u64)) });
    }
    return true;
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_5() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
