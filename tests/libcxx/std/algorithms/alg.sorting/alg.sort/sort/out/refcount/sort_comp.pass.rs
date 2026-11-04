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
impl CppMove for indirect_less {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
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
    {
        let v: Value<Vec<i32>> = Rc::new(RefCell::new(
            (0..(1000_usize) as usize)
                .map(|_| 0_i32)
                .collect::<Vec<_>>(),
        ));
        let mut i: i32 = 0;
        'loop_: while ((i as usize) < (*v.borrow()).len()) {
            let __rhs = i;
            v.decay().offset((i as usize) as isize).write(__rhs);
            i.prefix_inc();
        }
        ({
            let ___first: RandomAccessIterator<i32> = CppRange::begin(&v.decay());
            let ___last: RandomAccessIterator<i32> = CppRange::end(&v.decay());
            libcc2rs::sort_refcount(___first, ___last, <std_greater_int_>::default())
        });
        ({
            let ___first: RandomAccessIterator<i32> = CppRange::begin(&v.decay());
            let ___last: RandomAccessIterator<i32> = CppRange::end(&v.decay());
            reverse_0(___first, ___last)
        });
        assert!(
            ({
                let ___first: RandomAccessIterator<i32> = CppRange::begin(&v.decay());
                let ___last: RandomAccessIterator<i32> = CppRange::end(&v.decay());
                is_sorted_1(___first, ___last)
            })
        );
    }
    {
        let v: Value<Vec<Option<Value<i32>>>> = Rc::new(RefCell::new(
            (0..(1000_usize) as usize).map(|_| None).collect::<Vec<_>>(),
        ));
        let mut i: i32 = 0;
        'loop_: while ((i as usize) < (*v.borrow()).len()) {
            match (v.decay().offset((i as usize) as isize),) {
                (__tmp_0,) => {
                    let _p: Ptr<_> = Ptr::alloc(i);
                    __tmp_0.write(_p.to_owned_opt())
                }
            };
            i.prefix_inc();
        }
        ({
            let ___first: RandomAccessIterator<Option<Value<i32>>> = CppRange::begin(&v.decay());
            let ___last: RandomAccessIterator<Option<Value<i32>>> = CppRange::end(&v.decay());
            libcc2rs::sort_refcount(___first, ___last, <indirect_less>::default())
        });
        assert!(
            ({
                let ___first: RandomAccessIterator<Option<Value<i32>>> =
                    CppRange::begin(&v.decay());
                let ___last: RandomAccessIterator<Option<Value<i32>>> = CppRange::end(&v.decay());
                is_sorted_2(___first, ___last, <indirect_less>::default())
            })
        );
        assert!(
            ({
                (((*v.decay().offset(0_usize as isize).upgrade().deref())
                    .as_pointer()
                    .read())
                    == 0)
            })
        );
        assert!(
            ({
                (((*v.decay().offset(1_usize as isize).upgrade().deref())
                    .as_pointer()
                    .read())
                    == 1)
            })
        );
        assert!(
            ({
                (((*v.decay().offset(2_usize as isize).upgrade().deref())
                    .as_pointer()
                    .read())
                    == 2)
            })
        );
    }
    return 0;
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
