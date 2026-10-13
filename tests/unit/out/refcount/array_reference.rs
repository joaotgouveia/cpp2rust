extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn len_0(s: Ptr<i8>) -> i32 {
    let mut n: i32 = 0;
    'loop_: while (((elem!((s), n).read()) as i32) != (('\0' as i8) as i32)) {
        n.prefix_inc();
    }
    return n;
}
pub fn len5_1(mut s: Ptr<i8>) -> i32 {
    let mut n: i32 = 0;
    'loop_: while (((elem!(s, n).read()) as i32) != (('\0' as i8) as i32)) {
        n.prefix_inc();
    }
    return n;
}
pub fn sum_2(a: Ptr<i32>) -> i32 {
    return (((elem!((a), 0).read()) + (elem!((a), 1).read())) + (elem!((a), 2).read()));
}
pub fn fill_3(a: Ptr<i32>, mut v: i32) {
    let mut i: i32 = 0;
    'loop_: while (i < 3) {
        elem!((a), i).write(v);
        i.prefix_inc();
    }
}
pub fn sum_twice_4(a: Ptr<i32>) -> i32 {
    return (({ sum_2((a).clone()) }) + ({ sum_2((a).clone()) }));
}
pub fn sum_ptr_5(mut p: Ptr<i32>) -> i32 {
    return (((elem!(p, 0).read()) + (elem!(p, 1).read())) + (elem!(p, 2).read()));
}
pub fn sum_decayed_6(a: Ptr<i32>) -> i32 {
    return ({ sum_ptr_5((a).clone()) });
}
pub fn bump_ptr_7(mut p: Ptr<i32>) {
    {
        elem!(p, 0).with_mut(|__v| *__v = *__v + 1)
    };
}
pub fn bump_decayed_8(a: Ptr<i32>) {
    ({ bump_ptr_7((a).clone()) });
}
pub fn fill_and_sum_9(a: Ptr<i32>, mut v: i32, out: Ptr<i32>) {
    ({
        let _a: Ptr<i32> = (a).clone();
        let _v: i32 = v;
        fill_3(_a, _v)
    });
    let __rhs = ({ sum_twice_4((a).clone()) });
    out.write(__rhs);
}
pub fn pick_10(s: Ptr<i8>) -> Ptr<i8> {
    return (s).clone();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
pub fn sum_points_11(p: Ptr<Point>) -> i32 {
    return ({
        ({
            ({ { (*elem!((p), 0).upgrade().deref()).x } } + {
                { (*elem!((p), 0).upgrade().deref()).y }
            })
        } + { { (*elem!((p), 1).upgrade().deref()).x } })
    } + { { (*elem!((p), 1).upgrade().deref()).y } });
}
pub fn shift_points_12(p: Ptr<Point>, mut d: i32) {
    {
        let __rhs = d;
        field!(elem!((p), 0), x).with_mut(|__v| *__v = *__v + __rhs)
    };
    {
        let __rhs = d;
        field!(elem!((p), 1), y).with_mut(|__v| *__v = *__v + __rhs)
    };
}
pub fn total_len_13(names: Ptr<Ptr<i8>>) -> i32 {
    return (({ len5_1((elem!((names), 0).read())) }) + ({ len5_1((elem!((names), 1).read())) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    (if (({ len_0(Ptr::<i8>::from_string_literal(b"beta")) }) == 4) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"len(\"beta\") == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abcd\0")));
    (if (({ len_0(buf.as_pointer()) }) == 4) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"len(buf) == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    (if (({ sum_2(arr.as_pointer()) }) == 6) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"sum(arr) == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        let _a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
        sum_2(_a.as_pointer())
    }) == 6)
    {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"sum({1, 2, 3}) == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ fill_3(arr.as_pointer(), 7) });
    (if (({ sum_2(arr.as_pointer()) }) == 21) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"sum(arr) == 21"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ sum_twice_4(arr.as_pointer()) }) == 42) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"sum_twice(arr) == 42"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let out: Value<i32> = Rc::new(RefCell::new(0));
    ({ fill_and_sum_9(arr.as_pointer(), 2, out.as_pointer()) });
    (if ((*out.borrow()) == 12) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"out == 12"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((*arr.borrow())[(0) as usize] == 2) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"arr[0] == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let lit: Ptr<i8> = Ptr::<i8>::from_string_literal(b"beta");
    (if (({ len_0((lit).clone()) }) == 4) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"len(lit) == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((elem!(({ pick_10(Ptr::<i8>::from_string_literal(b"beta"),) }), 0).read()) as i32)
        == (('b' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"pick(\"beta\")[0] == \'b\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ len_0(({ pick_10(buf.as_pointer()) })) }) == 4) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"len(pick(buf)) == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let pts: Value<Box<[Point]>> = Rc::new(RefCell::new(Box::new([
        Point { x: 1, y: 2 },
        Point { x: 3, y: 4 },
    ])));
    (if (({ sum_points_11(pts.as_pointer()) }) == 10) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"sum_points(pts) == 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ shift_points_12(pts.as_pointer(), 10) });
    (if ({ (*pts.borrow())[(0) as usize].x } == 11) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"pts[0].x == 11"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*pts.borrow())[(1) as usize].y } == 14) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"pts[1].y == 14"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ sum_points_11(pts.as_pointer()) }) == 30) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"sum_points(pts) == 30"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ sum_decayed_6(arr.as_pointer()) }) == ({ sum_2(arr.as_pointer()) })) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"sum_decayed(arr) == sum(arr)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ bump_decayed_8(arr.as_pointer()) });
    (if ((*arr.borrow())[(0) as usize] == 3) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"arr[0] == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let names: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<i8>::from_string_literal(b"ab"),
        Ptr::<i8>::from_string_literal(b"cde"),
    ])));
    (if (({ total_len_13(names.as_pointer()) }) == 5) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"total_len(names) == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let rows: Value<Box<[Value<Box<[i32]>>]>> = Rc::new(RefCell::new(Box::new([
        Rc::new(RefCell::new(Box::new([1, 2, 3]))),
        Rc::new(RefCell::new(Box::new([4, 5, 6]))),
    ])));
    let mut p: Ptr<Value<Box<[i32]>>> = (rows.as_pointer() as Ptr<Value<Box<[i32]>>>);
    p.prefix_inc();
    (if ({ (p.read().as_pointer()) } == {
        ((((rows.as_pointer() as Ptr<Value<Box<[i32]>>>)
            .offset(1)
            .read()
            .as_pointer()) as Ptr<i32>)
            .offset(0))
    }) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"*p == &rows[1][0]"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let r: Ptr<i32> = (p.read().as_pointer());
    (if ((elem!((r), 0).read()) == 4) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"r[0] == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ sum_2((r).clone()) }) == 15) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"sum(r) == 15"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ sum_2((p.read().as_pointer())) }) == 15) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"sum(*p) == 15"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut q: Ptr<Value<Box<[i32]>>> = (rows.as_pointer() as Ptr<Value<Box<[i32]>>>);
    (if (({ sum_2(q.offset((1) as isize).read().as_pointer()) }) == 15) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"sum(q[1]) == 15"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (q.offset((1) as isize).read().as_pointer() as Ptr<i32>) } == {
        ((((rows.as_pointer() as Ptr<Value<Box<[i32]>>>)
            .offset(1)
            .read()
            .as_pointer()) as Ptr<i32>)
            .offset(0))
    }) {
        &(0);
    } else {
        ({
            __assert_fail_14(
                Ptr::<i8>::from_string_literal(b"q[1] == &rows[1][0]"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_14(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
