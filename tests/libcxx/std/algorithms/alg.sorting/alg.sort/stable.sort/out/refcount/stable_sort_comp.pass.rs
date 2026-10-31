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
pub struct indirect_less {}
impl Callable2<Ptr<Option<Value<i32>>>, Ptr<Option<Value<i32>>>, bool> for indirect_less {
    fn call(&self, a1: Ptr<Option<Value<i32>>>, a2: Ptr<Option<Value<i32>>>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            indirect_lessImpl::operator_call_1_class_std_unique_ptr_int_(__it, a1, a2)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct first_only {}
impl Callable2<Ptr<(Value<i32>, Value<i32>)>, Ptr<(Value<i32>, Value<i32>)>, bool> for first_only {
    fn call(&self, a1: Ptr<(Value<i32>, Value<i32>)>, a2: Ptr<(Value<i32>, Value<i32>)>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            first_onlyImpl::operator_call(__it, a1, a2)
        })
    }
}
pub fn generate_sawtooth_0(mut N: i32, mut M: i32) -> Vec<(Value<i32>, Value<i32>)> {
    let v: Value<Vec<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
        (0..(N as usize) as usize)
            .map(|_| (Rc::new(RefCell::new(0_i32)), Rc::new(RefCell::new(0_i32))))
            .collect::<Vec<_>>(),
    ));
    let mut x: i32 = 0;
    let mut ver: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        v.decay()
            .offset((i as usize) as isize)
            .write((Rc::new(RefCell::new(x)), Rc::new(RefCell::new(ver))));
        if (x.prefix_inc() == M) {
            x = 0;
            ver.prefix_inc();
        }
        i.prefix_inc();
    }
    return std::mem::take(&mut (*v.borrow_mut()));
}
pub fn test_1() -> bool {
    let mut N: i32 = 1000;
    let mut M: i32 = 10;
    {
        let v: Value<Vec<(Value<i32>, Value<i32>)>> =
            Rc::new(RefCell::new(({ generate_sawtooth_0(N, M) })));
        {
            let __first = v.decay().begin();
            let __n = v.decay().end().iter_distance(&__first);
            let __cmp = <first_only>::default();
            let mut __order: Vec<isize> = (0..__n).collect();
            __order.sort_by(|&__x, &__y| {
                let __a = __first.iter_at(__x);
                let __b = __first.iter_at(__y);
                if __cmp.less(__a.clone(), __b.clone()) {
                    std::cmp::Ordering::Less
                } else if __cmp.less(__b, __a) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            reorder(__order, |__i| __first.iter_at(__i));
        };
        assert!(unimplemented!());
    }
    {
        {
            let v: Value<Vec<(Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(({ generate_sawtooth_0(N, M) })));
            let randomness: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut i: i32 = 0;
            'loop_: while (i < (N - M)) {
                unimplemented!();
                i += M;
            }
            {
                let __first = v.decay().begin();
                let __n = v.decay().end().iter_distance(&__first);
                let __cmp = <first_only>::default();
                let mut __order: Vec<isize> = (0..__n).collect();
                __order.sort_by(|&__x, &__y| {
                    let __a = __first.iter_at(__x);
                    let __b = __first.iter_at(__y);
                    if __cmp.less(__a.clone(), __b.clone()) {
                        std::cmp::Ordering::Less
                    } else if __cmp.less(__b, __a) {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                });
                reorder(__order, |__i| __first.iter_at(__i));
            };
            assert!(unimplemented!());
        }
    }
    {
        let v: Value<Vec<Option<Value<i32>>>> = Rc::new(RefCell::new(
            (0..(1000_usize) as usize).map(|_| None).collect::<Vec<_>>(),
        ));
        let mut i: i32 = 0;
        'loop_: while ((i as usize) < (*v.borrow()).len()) {
            {
                let _p: Ptr<_> = Ptr::alloc(i);
                v.decay()
                    .offset((i as usize) as isize)
                    .write(_p.to_owned_opt())
            };
            i.prefix_inc();
        }
        {
            let __first = v.decay().begin();
            let __n = v.decay().end().iter_distance(&__first);
            let __cmp = <indirect_less>::default();
            let mut __order: Vec<isize> = (0..__n).collect();
            __order.sort_by(|&__x, &__y| {
                let __a = __first.iter_at(__x);
                let __b = __first.iter_at(__y);
                if __cmp.less(__a.clone(), __b.clone()) {
                    std::cmp::Ordering::Less
                } else if __cmp.less(__b, __a) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            reorder(__order, |__i| __first.iter_at(__i));
        };
        assert!(unimplemented!());
        assert!(
            (((*v.decay().offset(0_usize as isize).upgrade().deref())
                .as_pointer()
                .read())
                == 0)
        );
        assert!(
            (((*v.decay().offset(1_usize as isize).upgrade().deref())
                .as_pointer()
                .read())
                == 1)
        );
        assert!(
            (((*v.decay().offset(2_usize as isize).upgrade().deref())
                .as_pointer()
                .read())
                == 2)
        );
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
    ({ test_1() });
    return 0;
}
pub trait first_onlyImpl {
    fn operator_call(
        &self,
        x: Ptr<(Value<i32>, Value<i32>)>,
        y: Ptr<(Value<i32>, Value<i32>)>,
    ) -> bool;
}
impl first_onlyImpl for Ptr<first_only> {
    fn operator_call(
        &self,
        x: Ptr<(Value<i32>, Value<i32>)>,
        y: Ptr<(Value<i32>, Value<i32>)>,
    ) -> bool {
        return ({ (*(*x.upgrade().deref()).0.borrow()) } < {
            (*(*y.upgrade().deref()).0.borrow())
        });
    }
}
pub trait indirect_lessImpl {
    fn operator_call_1_class_std_unique_ptr_int_(
        &self,
        x: Ptr<Option<Value<i32>>>,
        y: Ptr<Option<Value<i32>>>,
    ) -> bool;
}
impl indirect_lessImpl for Ptr<indirect_less> {
    fn operator_call_1_class_std_unique_ptr_int_(
        &self,
        x: Ptr<Option<Value<i32>>>,
        y: Ptr<Option<Value<i32>>>,
    ) -> bool {
        return ({ ((*x.upgrade().deref()).as_pointer().read()) } < {
            ((*y.upgrade().deref()).as_pointer().read())
        });
    }
}
pub fn __cpp2rust_init_globals() {}
