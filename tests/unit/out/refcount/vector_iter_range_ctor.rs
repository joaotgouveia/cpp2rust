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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(i64, to_i64)]
#[byte_size(4)]
pub struct Reading {
    #[offset(0)]
    pub raw_: i32,
}
impl CppMove for Reading {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let src: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let v1: Value<Vec<u32>> = Rc::new(RefCell::new(
        match ({ (src.as_pointer() as Ptr<u32>).offset((3) as isize) },) {
            (mut __tmp_0,) => {
                let mut __v = Vec::new();
                let mut __it = (src.as_pointer() as Ptr<u32>);
                while __it != __tmp_0 {
                    __v.push((__it.iter_deref().read()));
                    __it.iter_inc();
                }
                __v
            }
        },
    ));
    assert!(({ ((*v1.borrow()).len() == 3_usize) }));
    assert!(
        (((v1.decay().offset(0_usize as isize).read()) == 1_u32)
            && ((v1.decay().offset(1_usize as isize).read()) == 2_u32))
            && ((v1.decay().offset(2_usize as isize).read()) == 3_u32)
    );
    let v2: Value<Vec<u64>> = Rc::new(RefCell::new(
        match ({ (src.as_pointer() as Ptr<u32>).offset((3) as isize) },) {
            (mut __tmp_1,) => {
                let mut __v = Vec::new();
                let mut __it = (src.as_pointer() as Ptr<u32>);
                while __it != __tmp_1 {
                    __v.push(((__it.iter_deref().read()) as u64));
                    __it.iter_inc();
                }
                __v
            }
        },
    ));
    assert!(({ ((*v2.borrow()).len() == 3_usize) }));
    assert!(
        (((v2.decay().offset(0_usize as isize).read()) == 1_u64)
            && ((v2.decay().offset(1_usize as isize).read()) == 2_u64))
            && ((v2.decay().offset(2_usize as isize).read()) == 3_u64)
    );
    let v3: Value<Vec<i32>> = Rc::new(RefCell::new(
        match ({ (src.as_pointer() as Ptr<u32>).offset((3) as isize) },) {
            (mut __tmp_2,) => {
                let mut __v = Vec::new();
                let mut __it = (src.as_pointer() as Ptr<u32>);
                while __it != __tmp_2 {
                    __v.push(((__it.iter_deref().read()) as i32));
                    __it.iter_inc();
                }
                __v
            }
        },
    ));
    assert!(({ ((*v3.borrow()).len() == 3_usize) }));
    assert!(
        (((v3.decay().offset(0_usize as isize).read()) == 1)
            && ((v3.decay().offset(1_usize as isize).read()) == 2))
            && ((v3.decay().offset(2_usize as isize).read()) == 3)
    );
    let src1: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let v4: Value<Vec<u32>> = Rc::new(RefCell::new(
        match ({ (src1.as_pointer() as Ptr<u32>).to_end() },) {
            (mut __tmp_3,) => {
                let mut __v = Vec::new();
                let mut __it = (src1.as_pointer() as Ptr<u32>);
                while __it != __tmp_3 {
                    __v.push((__it.iter_deref().read()));
                    __it.iter_inc();
                }
                __v
            }
        },
    ));
    assert!(({ ((*v4.borrow()).len() == 3_usize) }));
    assert!(
        (((v4.decay().offset(0_usize as isize).read()) == 1_u32)
            && ((v4.decay().offset(1_usize as isize).read()) == 2_u32))
            && ((v4.decay().offset(2_usize as isize).read()) == 3_u32)
    );
    let buf: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::new([10_u8, 20_u8, 30_u8, 40_u8, 50_u8])));
    let mut start: Ptr<u8> = (buf.as_pointer() as Ptr<u8>);
    let mut len: usize = 5_usize;
    let v5: Value<Vec<u8>> = Rc::new(RefCell::new(match ({ start.offset((len) as isize) },) {
        (mut __tmp_4,) => {
            let mut __v = Vec::new();
            let mut __it = (start).clone();
            while __it != __tmp_4 {
                __v.push((__it.iter_deref().read()));
                __it.iter_inc();
            }
            __v
        }
    }));
    assert!(({ ((*v5.borrow()).len() == 5_usize) }));
    assert!(
        (((v5.decay().offset(0_usize as isize).read()) as i32) == 10)
            && (((v5.decay().offset(4_usize as isize).read()) as i32) == 50)
    );
    let m: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(1)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(10);
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(2)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(20);
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(3)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(30);
    let v6: Value<Vec<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
        match (
            {
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
            },
            {
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            },
        ) {
            (mut __tmp_5, mut __tmp_6) => {
                let mut __v = Vec::new();
                let mut __it = (__tmp_5).clone();
                while __it != __tmp_6 {
                    __v.push(<(Value<i32>, Value<i32>) as CppFrom<(
                        Value<i32>,
                        Value<i32>,
                    )>>::cpp_from(__it.iter_deref()));
                    __it.iter_inc();
                }
                __v
            }
        },
    ));
    assert!(({ ((*v6.borrow()).len() == 3_usize) }));
    assert!(
        ({
            ((*(*v6.decay().offset(0_usize as isize).upgrade().deref())
                .0
                .borrow())
                == 1)
                && ((*(*v6.decay().offset(0_usize as isize).upgrade().deref())
                    .1
                    .borrow())
                    == 10)
        })
    );
    assert!(
        ({
            ((*(*v6.decay().offset(2_usize as isize).upgrade().deref())
                .0
                .borrow())
                == 3)
                && ((*(*v6.decay().offset(2_usize as isize).upgrade().deref())
                    .1
                    .borrow())
                    == 30)
        })
    );
    match (
        {
            <RandomAccessIterator<(Value<i32>, Value<i32>)>>::from(
                CppRange::begin(&v6.decay()).iter_add(1_isize),
            )
        },
        {
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
        },
        {
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
        },
    ) {
        (mut __tmp_7, mut __tmp_8, mut __tmp_9) => {
            let __start = __tmp_7.index();
            let mut __tmp: Vec<(Value<i32>, Value<i32>)> = Vec::new();
            let mut __it = (__tmp_8).clone();
            while __it != __tmp_9 {
                __tmp.push(<(Value<i32>, Value<i32>) as CppFrom<(
                    Value<i32>,
                    Value<i32>,
                )>>::cpp_from(__it.iter_deref()));
                __it.iter_inc();
            }
            v6.as_pointer()
                .with_mut(|__v: &mut Vec<(Value<i32>, Value<i32>)>| {
                    __v.splice(__start..__start, __tmp);
                });
            (__tmp_7).clone()
        }
    };
    assert!(({ ((*v6.borrow()).len() == 6_usize) }));
    assert!(
        ({
            (((*(*v6.decay().offset(1_usize as isize).upgrade().deref())
                .0
                .borrow())
                == 1)
                && ((*(*v6.decay().offset(3_usize as isize).upgrade().deref())
                    .0
                    .borrow())
                    == 3))
                && ((*(*v6.decay().offset(4_usize as isize).upgrade().deref())
                    .0
                    .borrow())
                    == 2)
        })
    );
    let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([4, 5, 6])));
    let v7: Value<Vec<Degrees>> = Rc::new(RefCell::new(
        match ({ (ints.as_pointer() as Ptr<i32>).offset((3) as isize) },) {
            (mut __tmp_10,) => {
                let mut __v = Vec::new();
                let mut __it = (ints.as_pointer() as Ptr<i32>);
                while __it != __tmp_10 {
                    __v.push(<Degrees as CppFrom<i32>>::cpp_from(__it.iter_deref()));
                    __it.iter_inc();
                }
                __v
            }
        },
    ));
    assert!((v7.decay().offset(1_usize as isize).with(|__s| __s.value) == 5));
    let readings: Value<Box<[Reading]>> = Rc::new(RefCell::new(Box::new([
        Reading { raw_: 4 },
        Reading { raw_: 5 },
    ])));
    let v8: Value<Vec<i64>> = Rc::new(RefCell::new(
        match ({ (readings.as_pointer() as Ptr<Reading>).offset((2) as isize) },) {
            (mut __tmp_11,) => {
                let mut __v = Vec::new();
                let mut __it = (readings.as_pointer() as Ptr<Reading>);
                while __it != __tmp_11 {
                    __v.push(<i64 as CppFrom<Reading>>::cpp_from(__it.iter_deref()));
                    __it.iter_inc();
                }
                __v
            }
        },
    ));
    assert!(
        ((v8.decay().offset(0_usize as isize).read()) == 40_i64)
            && ((v8.decay().offset(1_usize as isize).read()) == 50_i64)
    );
    return 0;
}
pub trait ReadingImpl {
    fn to_i64(&self) -> i64;
}
impl ReadingImpl for Ptr<Reading> {
    fn to_i64(&self) -> i64 {
        return (((*self).with(|__s| __s.raw_) as i64) * 10_i64);
    }
}
pub fn __cpp2rust_init_globals() {}
