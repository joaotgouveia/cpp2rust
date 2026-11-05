extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Bar {
    #[offset(0)]
    pub w: i32,
}
impl CppMove for Bar {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct Foo {
    #[offset(0)]
    pub x: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub y: Ptr<i32>,
    #[offset(16)]
    #[byte_size(8)]
    pub z: Ptr<i32>,
    #[offset(24)]
    #[byte_size(12)]
    pub a: Box<[i32]>,
    #[offset(36)]
    #[byte_size(4)]
    pub bar: Bar,
}
impl CppMove for Foo {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Foo {
    fn default() -> Self {
        {
            Foo {
                x: 0_i32,
                y: <Ptr<i32>>::default(),
                z: Ptr::<i32>::null(),
                a: (0..3).map(|_| 0_i32).collect::<Box<[i32]>>(),
                bar: <Bar>::default(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Refs {
    #[offset(0)]
    #[byte_size(8)]
    pub a: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub b: Ptr<i32>,
}
impl CppMove for Refs {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x1: Value<i32> = Rc::new(RefCell::new(1));
    let mut x2: i32 = (*x1.borrow());
    x2.prefix_inc();
    ({ cpp2rust_assert_fail_0(((*x1.borrow()) == 1)) });
    ({ cpp2rust_assert_fail_0((x2 == 2)) });
    let mut x3: f64 = 3_f64;
    let mut x4: f64 = x3;
    x4.prefix_inc();
    ({ cpp2rust_assert_fail_0((x3 == 3_f64)) });
    ({ cpp2rust_assert_fail_0((x4 == 4_f64)) });
    let reference: Ptr<i32> = x1.as_pointer();
    let mut x5: i32 = (reference.read());
    x5.prefix_inc();
    ({ cpp2rust_assert_fail_0(((reference.read()) == 1)) });
    ({ cpp2rust_assert_fail_0((x5 == 2)) });
    let mut pointer: Ptr<i32> = (x1.as_pointer());
    let mut x6: i32 = (pointer.read());
    x6.prefix_inc();
    ({ cpp2rust_assert_fail_0(((pointer.read()) == 1)) });
    ({ cpp2rust_assert_fail_0((x6 == 2)) });
    let mut other_pointer: Ptr<i32> = (pointer).clone();
    ({ cpp2rust_assert_fail_0(({ (other_pointer).clone() } == { (pointer).clone() })) });
    other_pointer.with_mut(|__v| {
        __v.prefix_inc();
    });
    ({ cpp2rust_assert_fail_0(({ (other_pointer.read()) } == { (pointer.read()) })) });
    let mut f1: Foo = Foo {
        x: 1,
        y: x1.as_pointer(),
        z: (x1.as_pointer()),
        a: Box::new([0, 1, 2]),
        bar: Bar { w: 10 },
    };
    ({ cpp2rust_assert_fail_0((f1.x == 1)) });
    ({ cpp2rust_assert_fail_0(((f1.y.read()) == 2)) });
    ({ cpp2rust_assert_fail_0(({ (f1.z).clone() } == { (x1.as_pointer()) })) });
    ({ cpp2rust_assert_fail_0(((f1.z.read()) == 2)) });
    let mut f2: Foo = (f1).clone();
    f2.x.prefix_inc();
    f2.y.with_mut(|__v| {
        __v.prefix_inc();
    });
    ({ cpp2rust_assert_fail_0((f2.x == 2)) });
    ({ cpp2rust_assert_fail_0(((f2.y.read()) == 3)) });
    ({ cpp2rust_assert_fail_0((f1.x == 1)) });
    ({ cpp2rust_assert_fail_0(((f1.y.read()) == 3)) });
    f2.z.with_mut(|__v| {
        __v.prefix_inc();
    });
    ({ cpp2rust_assert_fail_0(((f2.y.read()) == 4)) });
    ({ cpp2rust_assert_fail_0(({ (f2.z).clone() } == { (x1.as_pointer()) })) });
    ({ cpp2rust_assert_fail_0(((f2.z.read()) == 4)) });
    ({ cpp2rust_assert_fail_0(((f1.y.read()) == 4)) });
    ({ cpp2rust_assert_fail_0(({ (f1.z).clone() } == { (x1.as_pointer()) })) });
    ({ cpp2rust_assert_fail_0(((f1.z.read()) == 4)) });
    f2.a[(0) as usize].prefix_inc();
    f2.a[(1) as usize].prefix_inc();
    f2.a[(2) as usize].prefix_inc();
    ({ cpp2rust_assert_fail_0((f2.a[(0) as usize] == 1)) });
    ({ cpp2rust_assert_fail_0((f2.a[(1) as usize] == 2)) });
    ({ cpp2rust_assert_fail_0((f2.a[(2) as usize] == 3)) });
    ({ cpp2rust_assert_fail_0((f1.a[(0) as usize] == 0)) });
    ({ cpp2rust_assert_fail_0((f1.a[(1) as usize] == 1)) });
    ({ cpp2rust_assert_fail_0((f1.a[(2) as usize] == 2)) });
    f2.bar.w = 20;
    ({ cpp2rust_assert_fail_0((f2.bar.w == 20)) });
    ({ cpp2rust_assert_fail_0((f1.bar.w == 10)) });
    let mut N: i32 = 5;
    let v1: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__std_allocator_int__>::default(),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < N) {
        ({ (*v1.borrow()).push_back_59(i.as_pointer()) });
        (*i.borrow_mut()).prefix_inc();
    }
    let v2: Value<std_vector_int__std_allocator_int__> =
        Rc::new(RefCell::new((*v1.borrow()).clone()));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        ({ cpp2rust_assert_fail_0(({ (*v2.borrow())[(i as u64)] } == i)) });
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        (*v2.borrow_mut())[(i as u64)].prefix_inc();
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        ({ cpp2rust_assert_fail_0(({ (*v2.borrow())[(i as u64)] } == (i + 1))) });
        ({ cpp2rust_assert_fail_0(({ (*v1.borrow())[(i as u64)] } == i)) });
        i.prefix_inc();
    }
    let m1 : Value<std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ > = Rc::new(RefCell::new(<std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____  >::default() )) ;
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        ({
            let ___x: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
                std_vector_int__std_allocator_int__::new_1({ 10_u64 }, None),
            ));
            (*m1.borrow()).push_back_60(___x.as_pointer())
        });
        i.prefix_inc();
    }
    let m2 : Value<std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ > = Rc::new(RefCell::new(((*m1.borrow()) ).clone() )) ;
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        ({ cpp2rust_assert_fail_0((({ (*m1.borrow())[(i as u64)].size() }) == 10_u64)) });
        ({ cpp2rust_assert_fail_0((({ (*m2.borrow())[(i as u64)].size() }) == 10_u64)) });
        let mut j: i32 = 0;
        'loop_: while (j < 10) {
            ({
                cpp2rust_assert_fail_0(
                    ((elem!(m1.as_pointer().offset((i as u64)), (j as u64)).read()) == 0),
                )
            });
            ({
                cpp2rust_assert_fail_0(
                    ((elem!(m2.as_pointer().offset((i as u64)), (j as u64)).read()) == 0),
                )
            });
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        let mut j: i32 = 0;
        'loop_: while (j < 10) {
            elem!(m2.as_pointer().offset((i as u64)), (j as u64)).with_mut(|__v| __v.postfix_inc());
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        ({ cpp2rust_assert_fail_0((({ (*m1.borrow())[(i as u64)].size() }) == 10_u64)) });
        ({ cpp2rust_assert_fail_0((({ (*m2.borrow())[(i as u64)].size() }) == 10_u64)) });
        let mut j: i32 = 0;
        'loop_: while (j < 10) {
            ({
                cpp2rust_assert_fail_0(
                    ((elem!(m1.as_pointer().offset((i as u64)), (j as u64)).read()) == 0),
                )
            });
            ({
                cpp2rust_assert_fail_0(
                    ((elem!(m2.as_pointer().offset((i as u64)), (j as u64)).read()) == 1),
                )
            });
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let map1: Value<std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int___> =
        Rc::new(RefCell::new(
            <std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int___>::default(),
        ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < N) {
        let __rhs = (*i.borrow());
        (*map1.borrow_mut())[(*i.borrow())] = __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    let map2: Value<std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int___> =
        Rc::new(RefCell::new((*map1.borrow()).clone()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < N) {
        ({ cpp2rust_assert_fail_0(({ (*map2.borrow())[(*i.borrow())] } == (*i.borrow()))) });
        (*map2.borrow_mut())[(*i.borrow())].prefix_inc();
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < N) {
        ({ cpp2rust_assert_fail_0(({ (*map1.borrow())[(*i.borrow())] } == (*i.borrow()))) });
        ({ cpp2rust_assert_fail_0(({ (*map2.borrow())[(*i.borrow())] } == ((*i.borrow()) + 1))) });
        (*i.borrow_mut()).prefix_inc();
    }
    let pair1: Value<std_pair_int__int_> = Rc::new(RefCell::new({
        let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
        let __tmp_1: Value<i32> = Rc::new(RefCell::new(2));
        std_pair_int__int_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
    }));
    let pair2: Value<std_pair_int__int_> = Rc::new(RefCell::new(std_pair_int__int_::new_2({
        pair1.as_pointer()
    })));
    (*pair2.borrow_mut()).first = { ({ (*pair2.borrow()).first } * 10) };
    (*pair2.borrow_mut()).second = { ({ (*pair2.borrow()).second } * 10) };
    ({ cpp2rust_assert_fail_0(({ (*pair2.borrow()).first } == 10)) });
    ({ cpp2rust_assert_fail_0(({ (*pair2.borrow()).second } == 20)) });
    ({ cpp2rust_assert_fail_0(({ (*pair1.borrow()).first } == 1)) });
    ({ cpp2rust_assert_fail_0(({ (*pair1.borrow()).second } == 2)) });
    let pair3: Value<std_pair_std_vector_int__std_allocator_int____int_> = Rc::new(RefCell::new({
        let __tmp_2: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
            std_vector_int__std_allocator_int__::new_1({ 0_u64 }, None),
        ));
        let __tmp_3: Value<i32> = Rc::new(RefCell::new(0));
        std_pair_std_vector_int__std_allocator_int____int_::new_1({ __tmp_2.as_pointer() }, {
            __tmp_3.as_pointer()
        })
    }));
    let pair4: Value<std_pair_std_vector_int__std_allocator_int____int_> = Rc::new(RefCell::new(
        std_pair_std_vector_int__std_allocator_int____int_::new_2({ pair3.as_pointer() }),
    ));
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(1));
        (*pair4.borrow()).first.push_back_60(___x.as_pointer())
    });
    (*pair4.borrow_mut()).second = 1;
    ({ cpp2rust_assert_fail_0((({ (*pair4.borrow()).first.size() }) == 1_u64)) });
    ({ cpp2rust_assert_fail_0(({ (*pair4.borrow()).second } == 1)) });
    ({ cpp2rust_assert_fail_0((({ (*pair3.borrow()).first.size() }) == 0_u64)) });
    ({ cpp2rust_assert_fail_0(({ (*pair3.borrow()).second } == 0)) });
    let s1: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { 3_u64 },
                { ('a' as i8) },
                None,
            ),
        ));
    let mut s2: std_basic_string_char__std_char_traits_char___std_allocator_char__ =
        (*s1.borrow()).clone();
    s2[0_u64] = ('b' as i8);
    s2[1_u64] = ('b' as i8);
    s2[2_u64] = ('b' as i8);
    ({ cpp2rust_assert_fail_0(((s2[0_u64] as i32) == (('b' as i8) as i32))) });
    ({ cpp2rust_assert_fail_0(((s2[1_u64] as i32) == (('b' as i8) as i32))) });
    ({ cpp2rust_assert_fail_0(((s2[2_u64] as i32) == (('b' as i8) as i32))) });
    ({ cpp2rust_assert_fail_0((({ (*s1.borrow())[0_u64] } as i32) == (('a' as i8) as i32))) });
    ({ cpp2rust_assert_fail_0((({ (*s1.borrow())[1_u64] } as i32) == (('a' as i8) as i32))) });
    ({ cpp2rust_assert_fail_0((({ (*s1.borrow())[2_u64] } as i32) == (('a' as i8) as i32))) });
    let mut b1: Bar = Bar { w: 1 };
    let mut b2: Bar = Bar { w: 2 };
    b2 = (b1).clone();
    b2.w.postfix_inc();
    ({ cpp2rust_assert_fail_0((b1.w == 1)) });
    ({ cpp2rust_assert_fail_0((b2.w == 2)) });
    let v4: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__std_allocator_int__>::default(),
    ));
    (*v4.borrow_mut()) = (*v2.borrow()).clone();
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        ({ cpp2rust_assert_fail_0(({ (*v4.borrow())[(i as u64)] } == (i + 1))) });
        (*v4.borrow_mut())[(i as u64)].prefix_inc();
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        ({ cpp2rust_assert_fail_0(({ (*v4.borrow())[(i as u64)] } == (i + 2))) });
        ({ cpp2rust_assert_fail_0(({ (*v2.borrow())[(i as u64)] } == (i + 1))) });
        i.prefix_inc();
    }
    let ra: Value<i32> = Rc::new(RefCell::new(1));
    let rb: Value<i32> = Rc::new(RefCell::new(2));
    let mut r1: Refs = Refs {
        a: ra.as_pointer(),
        b: rb.as_pointer(),
    };
    let mut r2: Refs = (r1).clone();
    r2.a.write(10);
    r2.b.with_mut(|__v| {
        __v.prefix_inc();
    });
    ({ cpp2rust_assert_fail_0(((*ra.borrow()) == 10)) });
    ({ cpp2rust_assert_fail_0(((*rb.borrow()) == 3)) });
    ({ cpp2rust_assert_fail_0(((r1.a.read()) == 10)) });
    ({ cpp2rust_assert_fail_0(((r1.b.read()) == 3)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
