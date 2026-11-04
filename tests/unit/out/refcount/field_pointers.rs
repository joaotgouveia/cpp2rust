extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct Inner {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    #[byte_size(8)]
    pub name: Box<[i8]>,
}
impl CppMove for Inner {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Inner {
    fn default() -> Self {
        {
            Inner {
                a: 0_i32,
                name: (0..8).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Header {
    #[offset(0)]
    pub tag: i32,
    #[offset(4)]
    pub size: i16,
}
impl CppMove for Header {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(104)]
pub struct Outer {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    #[byte_size(12)]
    pub inner: Inner,
    #[offset(16)]
    #[byte_size(36)]
    pub items: Box<[Inner]>,
    #[offset(56)]
    #[byte_size(24)]
    pub v: std_vector_int__std_allocator_int__,
    #[offset(80)]
    #[byte_size(8)]
    pub cursor: Ptr<i32>,
    #[offset(88)]
    #[byte_size(16)]
    pub buf: Box<[i32]>,
}
impl Outer {
    pub fn move_from(_a0: Ptr<Outer>) -> Self {
        Self {
            x: { (*_a0.upgrade().deref()).x },
            inner: { (*_a0.upgrade().deref()).inner.clone() },
            items: Box::new(std::array::from_fn::<_, 3, _>(|__i: usize| {
                ((*_a0.upgrade().deref()).items[(__i) as usize]).clone()
            })),
            v: field!(_a0, v).with_mut(std::mem::take),
            cursor: { (*_a0.upgrade().deref()).cursor.clone() },
            buf: Box::new(std::array::from_fn::<_, 4, _>(|__i: usize| {
                (*_a0.upgrade().deref()).buf[(__i) as usize]
            })),
        }
    }
}
impl CppMove for Outer {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for Outer {
    fn default() -> Self {
        {
            Outer {
                x: 0_i32,
                inner: <Inner>::default(),
                items: (0..3).map(|_| <Inner>::default()).collect::<Box<[Inner]>>(),
                v: <std_vector_int__std_allocator_int__>::default(),
                cursor: Ptr::<i32>::null(),
                buf: (0..4).map(|_| 0_i32).collect::<Box<[i32]>>(),
            }
        }
    }
}
pub fn set_1(mut p: Ptr<i32>, mut value: i32) {
    p.write({ value });
}
pub fn bump_2(mut o: Ptr<Outer>) -> i32 {
    field!(o, x).with_mut(|__v| __v.postfix_inc());
    return 1;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Outer> = Rc::new(RefCell::new(<Outer>::default()));
    (*o.borrow_mut()).x = 1;
    let mut px: Ptr<i32> = (field_ptr!(o.as_pointer(), x));
    px.write(2);
    ({ cpp2rust_assert_fail_3(({ (*o.borrow()).x } == 2)) });
    let mut pa: Ptr<i32> = (field_ptr!(field_ptr!(o.as_pointer(), inner), a));
    ({
        let _p: Ptr<i32> = (pa).clone();
        set_1(_p, 3)
    });
    ({ cpp2rust_assert_fail_3(({ (*o.borrow()).inner.a } == 3)) });
    let mut pi: Ptr<i32> = (field_ptr!(
        (array_field_ptr!(o.as_pointer(), items) as Ptr<Inner>).offset((1) as isize),
        a
    ));
    pi.write(4);
    ({ cpp2rust_assert_fail_3((array_elem!(o.as_pointer(), items, 1).with(|__s| __s.a) == 4)) });
    ({
        cpp2rust_assert_fail_3(
            ({
                (field_ptr!(
                    (array_field_ptr!(o.as_pointer(), items) as Ptr<Inner>).offset((0) as isize),
                    a
                ))
            } != { (pi).clone() }),
        )
    });
    let mut name: Ptr<i8> = (array_field_ptr!(field_ptr!(o.as_pointer(), inner), name) as Ptr<i8>);
    let mut i: i32 = 0;
    'loop_: while (i < 3) {
        elem!(name, i).write({ (((('a' as i8) as i32) + i) as i8) });
        i.prefix_inc();
    }
    ({
        cpp2rust_assert_fail_3(
            (({
                strlen_4((array_field_ptr!(field_ptr!(o.as_pointer(), inner), name) as Ptr<i8>))
            }) == 3_u64),
        )
    });
    ({
        cpp2rust_assert_fail_3(
            ({ name.offset((3) as isize) } == {
                ((array_field_ptr!(field_ptr!(o.as_pointer(), inner), name) as Ptr<i8>)
                    .offset((3) as isize))
            }),
        )
    });
    (*o.borrow_mut()).cursor =
        { ((array_field_ptr!(o.as_pointer(), buf) as Ptr<i32>).offset((1) as isize)) };
    { (*o.borrow()).cursor.clone() }.write(5);
    {
        { (*o.borrow_mut()).cursor.postfix_inc() }.with_mut(|__v| *__v = *__v + 1)
    };
    ({
        cpp2rust_assert_fail_3(
            ((array_elem!(o.as_pointer(), buf, 1).read()) == 6)
                && ({ { (*o.borrow()).cursor.clone() } } == {
                    ((array_field_ptr!(o.as_pointer(), buf) as Ptr<i32>).offset((2) as isize))
                }),
        )
    });
    (*o.borrow_mut()).x = 0;
    let mut first: i32 = ({ OuterImpl::next(&o.as_pointer()) });
    ({ cpp2rust_assert_fail_3((first == 0) && ({ (*o.borrow()).x } == 1)) });
    let __rhs = ({ (*o.borrow()).inner.a } + ({ bump_2((o.as_pointer())) }));
    (*o.borrow_mut()).x = __rhs;
    ({ cpp2rust_assert_fail_3(({ (*o.borrow()).x } == 4)) });
    ({
        let _k: i32 = { (*o.borrow()).inner.a };
        OuterImpl::push(&o.as_pointer(), _k)
    });
    ({
        cpp2rust_assert_fail_3(
            (({ (*o.borrow()).v.size() }) == 1_u64)
                && ((elem!(field_ptr!(o.as_pointer(), v), 0_u64).read()) == 7),
        )
    });
    ({ cpp2rust_assert_fail_3((({ OuterImpl::sum(&o.as_pointer()) }) == 7)) });
    let y: Value<i32> = Rc::new(RefCell::new(0));
    ({
        let ___dest: AnyPtr = ((field_ptr!(
            (array_field_ptr!(o.as_pointer(), items) as Ptr<Inner>).offset((2) as isize),
            a
        )) as Ptr<i32>)
            .to_any();
        let ___src: AnyPtr = ((field_ptr!(
            (array_field_ptr!(o.as_pointer(), items) as Ptr<Inner>).offset((1) as isize),
            a
        )) as Ptr<i32>)
            .to_any();
        memcpy_5(___dest, ___src, (::std::mem::size_of::<i32>() as u64))
    });
    ({
        memcpy_5(
            ((y.as_pointer()) as Ptr<i32>).to_any(),
            ((field_ptr!(
                (array_field_ptr!(o.as_pointer(), items) as Ptr<Inner>).offset((2) as isize),
                a
            )) as Ptr<i32>)
                .to_any(),
            (::std::mem::size_of::<i32>() as u64),
        )
    });
    ({ cpp2rust_assert_fail_3(((*y.borrow()) == 4)) });
    let bytes: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
    ])));
    let mut view: Ptr<Header> = (bytes.as_pointer() as Ptr<u8>).reinterpret_cast::<Header>();
    field!(view, tag).write(16909060);
    ({
        cpp2rust_assert_fail_3(
            (((*bytes.borrow())[(0) as usize] as i32) == 4)
                || (((*bytes.borrow())[(3) as usize] as i32) == 4),
        )
    });
    ({ set_1((field_ptr!(view, tag)), 0) });
    ({
        cpp2rust_assert_fail_3(
            (((*bytes.borrow())[(0) as usize] as i32) == 0)
                && (((*bytes.borrow())[(3) as usize] as i32) == 0),
        )
    });
    field!(view, size).write(1_i16);
    ({
        cpp2rust_assert_fail_3(
            ((((*bytes.borrow())[(4) as usize] as i32) + ((*bytes.borrow())[(5) as usize] as i32))
                == 1),
        )
    });
    return 0;
}
pub trait OuterImpl {
    fn next(&self) -> i32;
    fn sum(&self) -> i32;
    fn push(&self, k: i32);
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer>;
}
impl OuterImpl for Ptr<Outer> {
    fn next(&self) -> i32 {
        return (array_elem!(
            (*self),
            buf,
            field!((*self), x).with_mut(|__v| __v.postfix_inc())
        )
        .read());
    }
    fn sum(&self) -> i32 {
        return ((*self).with(|__s| __s.x) + (*self).with(|__s| __s.inner.a));
    }
    fn push(&self, mut k: i32) {
        ({
            let ___x: Value<i32> = Rc::new(RefCell::new((k + (*self).with(|__s| __s.x))));
            (*(*self).upgrade().deref())
                .v
                .push_back_60(___x.as_pointer())
        });
    }
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer> {
        field!((*self), x).write({ { (*_a0.upgrade().deref()).x } });
        field!((*self), inner).write({ { (*_a0.upgrade().deref()).inner.clone() } });
        ({
            let _arg0: AnyPtr = ((array_field_ptr!((*self), items)) as Ptr<Inner>).to_any();
            let _arg1: AnyPtr = ((array_field_ptr!(_a0, items)) as Ptr<Inner>).to_any();
            __builtin_memcpy_0(_arg0, _arg1, 36_u64)
        });
        field!((*self), v).write({ { (*_a0.upgrade().deref()).v.clone() } });
        field!((*self), cursor).write({ { (*_a0.upgrade().deref()).cursor.clone() } });
        ({
            let _arg0: AnyPtr = ((array_field_ptr!((*self), buf)) as Ptr<i32>).to_any();
            let _arg1: AnyPtr = ((array_field_ptr!(_a0, buf)) as Ptr<i32>).to_any();
            __builtin_memcpy_0(_arg0, _arg1, 16_u64)
        });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
