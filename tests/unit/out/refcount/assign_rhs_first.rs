extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static g_cursor_0: Value<Ptr<Ptr<i32>>> = Rc::new(RefCell::new(Ptr::<Ptr<i32>>::null()));
);
pub fn advance_1() -> i32 {
    (*g_cursor_0.with(Value::clone).borrow()).with_mut(|__v| __v.prefix_inc());
    return 10;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct S {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<i32>,
}
pub fn by_ref_2(r: Ptr<Ptr<i32>>) {
    let __rhs = ({ advance_1() });
    {
        elem!((r.read()), 1).with_mut(|__v| *__v = *__v + __rhs)
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6, 7, 8])));
    let q: Value<Ptr<i32>> = Rc::new(RefCell::new((a.as_pointer() as Ptr<i32>)));
    g_cursor_0.with(|rc| *rc.borrow_mut() = (q.as_pointer()));
    let __rhs = ({ advance_1() });
    elem!((*q.borrow()), 1).write(__rhs);
    (if ({ (*q.borrow()).clone() } == { (a.as_pointer() as Ptr<i32>).offset((1) as isize) })
        && ((*a.borrow())[(2) as usize] == 10)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"q == a + 1 && a[2] == 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let __rhs = ({ advance_1() });
    {
        elem!((*q.borrow()), 1).with_mut(|__v| *__v = *__v + __rhs)
    };
    (if ({ (*q.borrow()).clone() } == { (a.as_pointer() as Ptr<i32>).offset((2) as isize) })
        && ((*a.borrow())[(3) as usize] == 14)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"q == a + 2 && a[3] == 14"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut pq: Ptr<Ptr<i32>> = (q.as_pointer());
    let __rhs = ({ advance_1() });
    {
        elem!((pq.read()), 1).with_mut(|__v| *__v = *__v + __rhs)
    };
    (if ({ (*q.borrow()).clone() } == { (a.as_pointer() as Ptr<i32>).offset((3) as isize) })
        && ((*a.borrow())[(4) as usize] == 15)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"q == a + 3 && a[4] == 15"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let __rhs = ({ advance_1() });
    elem!((pq.read()), 1).write(__rhs);
    (if ({ (*q.borrow()).clone() } == { (a.as_pointer() as Ptr<i32>).offset((4) as isize) })
        && ((*a.borrow())[(5) as usize] == 10)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"q == a + 4 && a[5] == 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ by_ref_2(q.as_pointer()) });
    (if ({ (*q.borrow()).clone() } == { (a.as_pointer() as Ptr<i32>).offset((5) as isize) })
        && ((*a.borrow())[(6) as usize] == 17)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"q == a + 5 && a[6] == 17"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let s: Value<S> = Rc::new(RefCell::new(S {
        ptr: (a.as_pointer() as Ptr<i32>),
    }));
    g_cursor_0.with(|rc| *rc.borrow_mut() = (field_ptr!(s.as_pointer(), ptr)));
    let __rhs = ({ advance_1() });
    {
        elem!({ (*s.borrow()).ptr.clone() }, 1).with_mut(|__v| *__v = *__v + __rhs)
    };
    (if ({ { (*s.borrow()).ptr.clone() } } == { (a.as_pointer() as Ptr<i32>).offset((1) as isize) })
        && ((*a.borrow())[(2) as usize] == 20)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"s.ptr == a + 1 && a[2] == 20"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut sp: Ptr<S> = (s.as_pointer());
    let __rhs = ({ advance_1() });
    {
        elem!(sp.with(|__s| __s.ptr.clone()), 1).with_mut(|__v| *__v = *__v + __rhs)
    };
    (if ({ { (*s.borrow()).ptr.clone() } } == { (a.as_pointer() as Ptr<i32>).offset((2) as isize) })
        && ((*a.borrow())[(3) as usize] == 24)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"s.ptr == a + 2 && a[3] == 24"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut b: Ptr<u8> = (a.as_pointer() as Ptr<i32>).reinterpret_cast::<u8>();
    elem!(b, 0).write(7_u8);
    elem!(b, 4).write({ (((elem!(b, 4).read()) as i32) + 1) as u8 });
    (if ((*a.borrow())[(0) as usize] == 7) && ((*a.borrow())[(1) as usize] == 3) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"a[0] == 7 && a[1] == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_3(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {
    let _ = g_cursor_0.with(|_| ());
}
