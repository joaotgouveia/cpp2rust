extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn set_0(mut p: Ptr<i32>, mut v: i32) {
    p.write({ v });
}
pub fn inc_1(r: Ptr<i32>) {
    r.with_mut(|__v| {
        __v.prefix_inc();
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
impl CppMove for Pair {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn total_2(v: Ptr<std_vector_int__std_allocator_int__>) -> i32 {
    let s: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: for x in IterRange::new(
        ({ (*v.upgrade().deref()).begin_29() }),
        ({ (*v.upgrade().deref()).end_31() }),
    ) {
        let mut x: i32 = (x.read());
        (*s.borrow_mut()) += x;
    }
    return (*s.borrow());
}
pub fn sum_3(mut arr: Ptr<i32>, mut n: i32) -> i32 {
    let s: Value<i32> = Rc::new(RefCell::new(0));
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        (*s.borrow_mut()) += { (elem!(arr, i).read()) };
        i.prefix_inc();
    }
    return (*s.borrow());
}
pub fn countdown_4(mut n: i32, step: Option<i32>) -> i32 {
    let mut step: i32 = step.unwrap_or_else(|| 1);
    let steps: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while (n > 0) {
        n -= step;
        (*steps.borrow_mut()).postfix_inc();
    }
    return (*steps.borrow());
}
pub trait Shape {
    fn scale(&mut self, factor: i32) -> i32;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct Square {
    #[offset(8)]
    pub side: i32,
}
impl CppMove for Square {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Square {
    fn default() -> Self {
        { Square { side: 2 } }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut plain: i32 = 1;
    plain += 2;
    plain.postfix_inc();
    ({ cpp2rust_assert_fail_5((plain == 4)) });
    &(::std::mem::size_of::<i32>());
    let addr: Value<i32> = Rc::new(RefCell::new(0));
    ({ set_0((addr.as_pointer()), 5) });
    ({ cpp2rust_assert_fail_5(((*addr.borrow()) == 5)) });
    let ref_: Value<i32> = Rc::new(RefCell::new(1));
    ({ inc_1(ref_.as_pointer()) });
    let alias: Ptr<i32> = ref_.as_pointer();
    alias.with_mut(|__v| __v.postfix_inc());
    ({ cpp2rust_assert_fail_5(((*ref_.borrow()) == 3)) });
    let captured: Value<i32> = Rc::new(RefCell::new(7));
    let get: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let captured: Ptr<i32> = captured.as_pointer();
        },
        || -> i32 {
            return (captured.read());
        }
    )));
    (*captured.borrow_mut()) = 8;
    ({ cpp2rust_assert_fail_5((({ (*get.borrow()).call() }) == 8)) });
    let mut arr: [i32; 4] = [1, 2, 3, 4];
    arr[(0) as usize] = { (arr[(3) as usize] * 2) };
    arr[(1) as usize].postfix_inc();
    ({ cpp2rust_assert_fail_5(((arr[(0) as usize] + arr[(1) as usize]) == 11)) });
    let elem: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0])));
    ({ set_0(((elem.as_pointer() as Ptr<i32>).offset(1)), 6) });
    ({ cpp2rust_assert_fail_5(((*elem.borrow())[(1) as usize] == 6)) });
    let decayed: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    ({ cpp2rust_assert_fail_5((({ sum_3((decayed.as_pointer() as Ptr<i32>), 3) }) == 6)) });
    let elem_ref: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1])));
    ({ inc_1((elem_ref.as_pointer() as Ptr<i32>).offset(0)) });
    ({ cpp2rust_assert_fail_5(((*elem_ref.borrow())[(0) as usize] == 2)) });
    let mut p: Ptr<i32> = (addr.as_pointer());
    p = ((elem.as_pointer() as Ptr<i32>).offset(0));
    p.write(9);
    ({ cpp2rust_assert_fail_5(((*elem.borrow())[(0) as usize] == 9)) });
    let mut str: [i8; 8] = b"abc\0\0\0\0\0".map(i8::from_byte);
    str[(0) as usize] = ('x' as i8);
    ({
        cpp2rust_assert_fail_5(
            ((str[(0) as usize] as i32) == (('x' as i8) as i32))
                && ((str[(3) as usize] as i32) == (('\0' as i8) as i32)),
        )
    });
    let mut zeros: [f64; 16] = [
        0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64,
        0_f64, 0_f64, 0_f64,
    ];
    zeros[(15) as usize] = 1.5_f64;
    ({
        cpp2rust_assert_fail_5((zeros[(0) as usize] == 0_f64) && (zeros[(15) as usize] == 1.5_f64))
    });
    ({ cpp2rust_assert_fail_5((({ countdown_4(10, None) }) == 10)) });
    ({ cpp2rust_assert_fail_5((({ countdown_4(10, Some(3)) }) == 4)) });
    let mut init: i32 = 3;
    let mut pair: Pair = Pair {
        first: init,
        second: (init + 1),
    };
    let mut heap: Ptr<Pair> = Ptr::alloc(Pair {
        first: init,
        second: init,
    });
    ({ cpp2rust_assert_fail_5((pair.second == 4) && (heap.with(|__s| __s.first) == 3)) });
    heap.delete();
    let vec_: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new({
        let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
        std_vector_int__std_allocator_int__::new_1({ 3_u64 }, { __tmp_0.as_pointer() }, None)
    }));
    let four: Value<i32> = Rc::new(RefCell::new(4));
    ({ (*vec_.borrow()).push_back_59(four.as_pointer()) });
    let __rhs = ({ (*vec_.borrow())[3_u64] } + 1);
    (*vec_.borrow_mut())[0_u64] = __rhs;
    (*vec_.borrow_mut())[1_u64].postfix_inc();
    ({
        cpp2rust_assert_fail_5(
            ((({ (*vec_.borrow()).size() }) == 4_u64) && ({ (*vec_.borrow())[0_u64] } == 5))
                && ({ (*vec_.borrow())[1_u64] } == 2),
        )
    });
    let high: Value<i32> = Rc::new(RefCell::new(3));
    let mut low: i32 = (({
        let ___b: Value<i32> = Rc::new(RefCell::new(2));
        min_6(high.as_pointer(), ___b.as_pointer())
    })
    .read());
    ({ cpp2rust_assert_fail_5((low == 2)) });
    let by_ref: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new({
        let __tmp_1: Value<i32> = Rc::new(RefCell::new(3));
        std_vector_int__std_allocator_int__::new_1({ 2_u64 }, { __tmp_1.as_pointer() }, None)
    }));
    ({ cpp2rust_assert_fail_5((({ total_2(by_ref.as_pointer()) }) == 6)) });
    let elem_ptr: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new({
        let __tmp_2: Value<i32> = Rc::new(RefCell::new(0));
        std_vector_int__std_allocator_int__::new_1({ 2_u64 }, { __tmp_2.as_pointer() }, None)
    }));
    ({ set_0((elem_ptr.as_pointer().offset(1_u64)), 7) });
    ({ cpp2rust_assert_fail_5(({ (*elem_ptr.borrow())[1_u64] } == 7)) });
    let data: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new({
        let __tmp_3: Value<i32> = Rc::new(RefCell::new(0));
        std_vector_int__std_allocator_int__::new_1({ 2_u64 }, { __tmp_3.as_pointer() }, None)
    }));
    ({ set_0(({ (*data.borrow()).data_57() }), 8) });
    ({ cpp2rust_assert_fail_5(({ (*data.borrow())[0_u64] } == 8)) });
    let square: Value<Square> = Rc::new(RefCell::new(<Square>::default()));
    let mut shape: PtrDyn<dyn Shape> = (square.as_pointer()).to_dyn::<dyn Shape>(|w| w);
    ({ cpp2rust_assert_fail_5((({ (*shape.upgrade().deref_mut()).scale(3) }) == 6)) });
    return 0;
}
impl Shape for Square {
    fn scale(&mut self, mut factor: i32) -> i32 {
        return (factor * { self.side });
    }
}
pub fn __cpp2rust_init_globals() {}
