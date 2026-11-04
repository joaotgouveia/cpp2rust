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
pub struct TestAllocator_int_ {}
impl CppMove for TestAllocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestAllocator_double_ {}
impl CppMove for TestAllocator_double_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn copy_0(mut copy_vector: std_vector_int__TestAllocator_int__) {}
pub fn fn_1(v: Ptr<std_vector_int__TestAllocator_int__>, v3: std_vector_int__TestAllocator_int__) {
    let v3: Value<std_vector_int__TestAllocator_int__> = Rc::new(RefCell::new(v3));
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(20));
        (*v.upgrade().deref()).push_back_60(___x.as_pointer())
    });
    let mut x: i32 = 0_i32;
    let mut v4: Ptr<std_vector_int__TestAllocator_int__> = (v3.as_pointer());
    let v2: Value<std_vector_int__TestAllocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__TestAllocator_int__>::default(),
    ));
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(0));
        (*v2.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(1));
        (*v2.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(3));
        (*v2.borrow()).push_back_60(___x.as_pointer())
    });
    x = (elem!(v, 2_u64).read());
    (*v2.borrow_mut())[0_u64] = 1;
    elem!((if true { (v3.as_pointer()) } else { (v).clone() }), 0_u64).write(7);
    elem!((v4), 1_u64).write(13);
    ({ cpp2rust_assert_fail_2((x == 6)) });
    ({ cpp2rust_assert_fail_2(((({ (*v.upgrade().deref()).front_53() }).read()) == 4)) });
    ({ cpp2rust_assert_fail_2(((elem!(v, 1_u64).read()) == 5)) });
    ({ cpp2rust_assert_fail_2(((elem!(v, 2_u64).read()) == 6)) });
    ({ cpp2rust_assert_fail_2(((({ (*v.upgrade().deref()).back_55() }).read()) == 20)) });
    ({ cpp2rust_assert_fail_2(({ (*v3.borrow())[0_u64] } == 7)) });
    ({ cpp2rust_assert_fail_2(({ (*v3.borrow())[1_u64] } == 13)) });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(20));
        (*v.upgrade().deref()).push_back_60(___x.as_pointer())
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v1: Value<std_vector_int__TestAllocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__TestAllocator_int__>::default(),
    ));
    ({ cpp2rust_assert_fail_2((({ (*v1.borrow()).size() }) == 0_u64)) });
    ({ cpp2rust_assert_fail_2(({ (*v1.borrow()).empty() })) });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(1));
        (*v1.borrow()).push_back_60(___x.as_pointer())
    });
    ({ cpp2rust_assert_fail_2(!({ (*v1.borrow()).empty() })) });
    ({ (*v1.borrow()).pop_back() });
    ({ cpp2rust_assert_fail_2(({ (*v1.borrow()).empty() })) });
    let mut s1: u64 = ({ (*v1.borrow()).size() });
    ({ (*v1.borrow()).resize_42(100_u64) });
    ({ cpp2rust_assert_fail_2((({ (*v1.borrow()).size() }) == 100_u64)) });
    ({ cpp2rust_assert_fail_2(({ (*v1.borrow())[99_u64] } == 0)) });
    (*v1.borrow_mut())[0_u64] = 40;
    (*v1.borrow_mut())[99_u64] = 50;
    ({ cpp2rust_assert_fail_2(({ (*v1.borrow())[0_u64] } == 40)) });
    ({ cpp2rust_assert_fail_2(({ (*v1.borrow())[99_u64] } == 50)) });
    let v2: Value<std_vector_int__TestAllocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__TestAllocator_int__>::default(),
    ));
    ({ cpp2rust_assert_fail_2((({ (*v2.borrow()).size() }) == 0_u64)) });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(1));
        (*v2.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(2));
        (*v2.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(3));
        (*v2.borrow()).push_back_60(___x.as_pointer())
    });
    ({ cpp2rust_assert_fail_2((({ (*v2.borrow()).size() }) == 3_u64)) });
    ({
        let ___position: __gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__TestAllocator_int___   = { let __tmp_0 : Value<__gnu_cxx___normal_iterator_int_ptr__std_vector_int__TestAllocator_int___ > = Rc::new(RefCell::new(( { (*v2.borrow()) . begin_28 ( ) } )  )); __gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__TestAllocator_int___ :: new_1 ( { __tmp_0.as_pointer() } , )  }  ;
        (*v2.borrow()).erase_71(___position)
    });
    ({ cpp2rust_assert_fail_2((({ (*v2.borrow()).size() }) == 2_u64)) });
    ({ cpp2rust_assert_fail_2(({ (*v2.borrow())[0_u64] } == 2)) });
    ({ cpp2rust_assert_fail_2(({ (*v2.borrow())[1_u64] } == 3)) });
    ({
        let ___position: __gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__TestAllocator_int___   = { let __tmp_1 : Value<__gnu_cxx___normal_iterator_int_ptr__std_vector_int__TestAllocator_int___ > = Rc::new(RefCell::new(( { (*v2.borrow()) . begin_28 ( ) } )  )); __gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__TestAllocator_int___ :: new_1 ( { __tmp_1.as_pointer() } , )  }  ;
        let ___x: Value<i32> = Rc::new(RefCell::new(100));
        (*v2.borrow()).insert_65(___position, ___x.as_pointer())
    });
    ({ copy_0((*v2.borrow()).clone()) });
    ({ cpp2rust_assert_fail_2((({ (*v2.borrow()).size() }) == 3_u64)) });
    ({ cpp2rust_assert_fail_2(({ (*v2.borrow())[0_u64] } == 100)) });
    ({ cpp2rust_assert_fail_2(({ (*v2.borrow())[1_u64] } == 2)) });
    ({ cpp2rust_assert_fail_2(({ (*v2.borrow())[2_u64] } == 3)) });
    let mut s2: u64 = ({ (*v2.borrow()).size() });
    let v3: Value<std_vector_int__TestAllocator_int__> = Rc::new(RefCell::new({
        let __tmp_2: Value<i32> = Rc::new(RefCell::new(1));
        std_vector_int__TestAllocator_int__::new_1({ 100_u64 }, { __tmp_2.as_pointer() }, None)
    }));
    ({ cpp2rust_assert_fail_2((({ (*v3.borrow()).size() }) == 100_u64)) });
    let mut i: i32 = 0;
    'loop_: while (i < 100) {
        ({ cpp2rust_assert_fail_2(({ (*v3.borrow())[(i as u64)] } == 1)) });
        i.prefix_inc();
    }
    let v6: Value<std_vector_double__TestAllocator_double__> = Rc::new(RefCell::new({
        let __tmp_3: Value<f64> = Rc::new(RefCell::new(2_f64));
        std_vector_double__TestAllocator_double__::new_1({ s2 }, { __tmp_3.as_pointer() }, None)
    }));
    ({ cpp2rust_assert_fail_2((({ (*v6.borrow()).size() }) == s2)) });
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < s2) {
        ({ cpp2rust_assert_fail_2(({ (*v6.borrow())[(i as u64)] } == 2_f64)) });
        i.prefix_inc();
    }
    let mut p1: Ptr<f64> = ({ (*v6.borrow()).data_57() });
    ({ cpp2rust_assert_fail_2(((p1.read()) == 2_f64)) });
    let mut p2: Ptr<i32> = ({ (*v3.borrow()).data_57() });
    ({ cpp2rust_assert_fail_2(((p2.read()) == 1)) });
    ({ cpp2rust_assert_fail_2(({ (*v3.borrow())[0_u64] } == 1)) });
    ({ cpp2rust_assert_fail_2(({ (*v3.borrow())[1_u64] } == 1)) });
    p2.write((99_f64 as i32));
    ({ cpp2rust_assert_fail_2(((p2.read()) == 99)) });
    ({ cpp2rust_assert_fail_2(({ (*v3.borrow())[0_u64] } == 99)) });
    ({ cpp2rust_assert_fail_2(({ (*v3.borrow())[1_u64] } == 1)) });
    p2.prefix_inc();
    p2.write(98);
    ({ cpp2rust_assert_fail_2(({ (*v3.borrow())[0_u64] } == 99)) });
    ({ cpp2rust_assert_fail_2(({ (*v3.borrow())[1_u64] } == 98)) });
    ({ cpp2rust_assert_fail_2((({ (*v3.borrow()).capacity() }) == 100_u64)) });
    ({ cpp2rust_assert_fail_2((({ (*v3.borrow()).size() }) == 100_u64)) });
    ({ (*v3.borrow()).reserve(200_u64) });
    ({ cpp2rust_assert_fail_2((({ (*v3.borrow()).capacity() }) == 200_u64)) });
    ({ cpp2rust_assert_fail_2((({ (*v3.borrow()).size() }) == 100_u64)) });
    ({ (*v3.borrow()).reserve(50_u64) });
    ({ cpp2rust_assert_fail_2((({ (*v3.borrow()).capacity() }) == 200_u64)) });
    ({ cpp2rust_assert_fail_2((({ (*v3.borrow()).size() }) == 100_u64)) });
    ({ (*v3.borrow()).reserve(200_u64) });
    ({ cpp2rust_assert_fail_2((({ (*v3.borrow()).capacity() }) == 200_u64)) });
    ({ cpp2rust_assert_fail_2((({ (*v3.borrow()).size() }) == 100_u64)) });
    ({ (*v3.borrow()).reserve(201_u64) });
    ({ cpp2rust_assert_fail_2((({ (*v3.borrow()).capacity() }) == 201_u64)) });
    ({ cpp2rust_assert_fail_2((({ (*v3.borrow()).size() }) == 100_u64)) });
    ({ cpp2rust_assert_fail_2(((({ (*v2.borrow()).back_55() }).read()) == 3)) });
    ({ cpp2rust_assert_fail_2(((({ (*v3.borrow()).back_55() }).read()) == 1)) });
    ({ cpp2rust_assert_fail_2(((({ (*v6.borrow()).back_55() }).read()) == 2_f64)) });
    let ref0: Ptr<f64> = ({ (*v6.borrow()).back_55() });
    ref0.write(5_f64);
    ({ cpp2rust_assert_fail_2(((({ (*v6.borrow()).back_55() }).read()) == 5_f64)) });
    let mut x0: f64 = (({ (*v6.borrow()).back_55() }).read());
    ({ cpp2rust_assert_fail_2((x0 == 5_f64)) });
    x0 = 6_f64;
    ({ cpp2rust_assert_fail_2(((({ (*v6.borrow()).back_55() }).read()) == 5_f64)) });
    let mut idx: i32 = 0;
    ({ cpp2rust_assert_fail_2(((({ (*v6.borrow()).at_51((idx as u64)) }).read()) == 2_f64)) });
    ({
        cpp2rust_assert_fail_2(
            ((({ (*v6.borrow()).at_51((s2).wrapping_sub(1_u64)) }).read()) == 5_f64),
        )
    });
    let ref1: Ptr<f64> = ({ (*v6.borrow()).at_51((s2).wrapping_sub(1_u64)) });
    {
        ref1.with_mut(|__v| *__v = *__v + 1.5_f64)
    };
    ({
        cpp2rust_assert_fail_2(
            ((({ (*v6.borrow()).at_51((s2).wrapping_sub(1_u64)) }).read()) == 6.5_f64),
        )
    });
    let mut x1: f64 = (({ (*v6.borrow()).at_51((s2).wrapping_sub(1_u64)) }).read());
    ({ cpp2rust_assert_fail_2((x1 == 6.5_f64)) });
    x1 -= 1.5_f64;
    ({
        cpp2rust_assert_fail_2(
            ((({ (*v6.borrow()).at_51((s2).wrapping_sub(1_u64)) }).read()) == 6.5_f64),
        )
    });
    let v7: Value<std_vector_int__TestAllocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__TestAllocator_int__>::default(),
    ));
    let v8: Value<std_vector_int__TestAllocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__TestAllocator_int__>::default(),
    ));
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(4));
        (*v7.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(5));
        (*v7.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(6));
        (*v7.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(8));
        (*v8.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(9));
        (*v8.borrow()).push_back_60(___x.as_pointer())
    });
    ({ fn_1(v7.as_pointer(), (*v8.borrow()).clone()) });
    let src: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let v9: Value<std_vector_unsigned_int__std_allocator_unsigned_int__> = Rc::new(RefCell::new(
        std_vector_unsigned_int__std_allocator_unsigned_int__::new_1(
            { (src.as_pointer() as Ptr<u32>) },
            { (src.as_pointer() as Ptr<u32>).offset((3) as isize) },
            None,
        ),
    ));
    ({ cpp2rust_assert_fail_2((({ (*v9.borrow()).size() }) == 3_u64)) });
    ({
        cpp2rust_assert_fail_2(
            (({ (*v9.borrow())[0_u64] } == 1_u32) && ({ (*v9.borrow())[1_u64] } == 2_u32))
                && ({ (*v9.borrow())[2_u64] } == 3_u32),
        )
    });
    let v10: Value<std_vector_unsigned_long__std_allocator_unsigned_long__> =
        Rc::new(RefCell::new(
            std_vector_unsigned_long__std_allocator_unsigned_long__::new_1(
                { (src.as_pointer() as Ptr<u32>) },
                { (src.as_pointer() as Ptr<u32>).offset((3) as isize) },
                None,
            ),
        ));
    ({ cpp2rust_assert_fail_2((({ (*v10.borrow()).size() }) == 3_u64)) });
    ({
        cpp2rust_assert_fail_2(
            (({ (*v10.borrow())[0_u64] } == 1_u64) && ({ (*v10.borrow())[1_u64] } == 2_u64))
                && ({ (*v10.borrow())[2_u64] } == 3_u64),
        )
    });
    let v11: Value<std_vector_int__std_allocator_int__> =
        Rc::new(RefCell::new(std_vector_int__std_allocator_int__::new_1(
            { (src.as_pointer() as Ptr<u32>) },
            { (src.as_pointer() as Ptr<u32>).offset((3) as isize) },
            None,
        )));
    ({ cpp2rust_assert_fail_2((({ (*v11.borrow()).size() }) == 3_u64)) });
    ({
        cpp2rust_assert_fail_2(
            (({ (*v11.borrow())[0_u64] } == 1) && ({ (*v11.borrow())[1_u64] } == 2))
                && ({ (*v11.borrow())[2_u64] } == 3),
        )
    });
    let src1: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let v12: Value<std_vector_unsigned_int__std_allocator_unsigned_int__> = Rc::new(RefCell::new(
        std_vector_unsigned_int__std_allocator_unsigned_int__::new_2(
            { (src1.as_pointer() as Ptr<u32>) },
            { ({ end_3(src1.as_pointer()) }) },
            None,
        ),
    ));
    ({ cpp2rust_assert_fail_2((({ (*v12.borrow()).size() }) == 3_u64)) });
    ({
        cpp2rust_assert_fail_2(
            (({ (*v12.borrow())[0_u64] } == 1_u32) && ({ (*v12.borrow())[1_u64] } == 2_u32))
                && ({ (*v12.borrow())[2_u64] } == 3_u32),
        )
    });
    let buf: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::new([10_u8, 20_u8, 30_u8, 40_u8, 50_u8])));
    let mut start: Ptr<u8> = (buf.as_pointer() as Ptr<u8>);
    let mut len: u64 = 5_u64;
    let v13: Value<std_vector_unsigned_char__std_allocator_unsigned_char__> =
        Rc::new(RefCell::new(
            std_vector_unsigned_char__std_allocator_unsigned_char__::new_1(
                { (start).clone() },
                { start.offset((len) as isize) },
                None,
            ),
        ));
    ({ cpp2rust_assert_fail_2((({ (*v13.borrow()).size() }) == 5_u64)) });
    ({
        cpp2rust_assert_fail_2(
            (({ (*v13.borrow())[0_u64] } as i32) == 10)
                && (({ (*v13.borrow())[4_u64] } as i32) == 50),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            (((s1).wrapping_add(s2))
                .wrapping_add(((({ (*v2.borrow()).at_51(0_u64) }).read()) as u64))
                == 103_u64),
        )
    });
    return 0;
}
pub trait TestAllocator_double_Impl {
    fn allocate(&self, n: u64) -> Ptr<f64>;
    fn deallocate(&self, p: Ptr<f64>, _a1: u64);
}
impl TestAllocator_double_Impl for Ptr<TestAllocator_double_> {
    fn allocate(&self, mut n: u64) -> Ptr<f64> {
        return Ptr::alloc_array((0..n).map(|_| 0_f64).collect::<Box<[f64]>>());
    }
    fn deallocate(&self, mut p: Ptr<f64>, mut _a1: u64) {
        p.delete();
    }
}
pub trait TestAllocator_int_Impl {
    fn allocate(&self, n: u64) -> Ptr<i32>;
    fn deallocate(&self, p: Ptr<i32>, _a1: u64);
}
impl TestAllocator_int_Impl for Ptr<TestAllocator_int_> {
    fn allocate(&self, mut n: u64) -> Ptr<i32> {
        return Ptr::alloc_array((0..n).map(|_| 0_i32).collect::<Box<[i32]>>());
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut _a1: u64) {
        p.delete();
    }
}
pub fn __cpp2rust_init_globals() {}
