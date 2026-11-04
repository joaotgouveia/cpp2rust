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
pub struct Stats {
    #[offset(0)]
    pub compared: i32,
    #[offset(4)]
    pub copied: i32,
    #[offset(8)]
    pub moved: i32,
}
impl CppMove for Stats {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Stats {
    fn default() -> Self {
        {
            Stats {
                compared: 0,
                copied: 0,
                moved: 0,
            }
        }
    }
}
thread_local!(
    pub static stats_0: Value<Stats> = Rc::new(RefCell::new(<Stats>::default()));
);
pub fn operator_lt_1(a: Ptr<MyInt>, b: Ptr<MyInt>) -> bool {
    (*stats_0.with(Value::clone).borrow_mut())
        .compared
        .prefix_inc();
    return ({ a.with(|__s| __s.value) } < { b.with(|__s| __s.value) });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign, Default)]
#[byte_size(4)]
pub struct MyInt {
    #[offset(0)]
    pub value: i32,
}
impl MyInt {
    pub fn new(mut xval: i32) -> Self {
        Self { value: xval }
    }
    pub fn copy_from(other: Ptr<MyInt>) -> Self {
        let __this: MyInt = Self {
            value: other.with(|__s| __s.value),
        };
        (*stats_0.with(Value::clone).borrow_mut())
            .copied
            .prefix_inc();
        __this
    }
    pub fn move_from(other: Ptr<MyInt>) -> Self {
        let __this: MyInt = Self {
            value: other.with(|__s| __s.value),
        };
        (*stats_0.with(Value::clone).borrow_mut())
            .moved
            .prefix_inc();
        __this
    }
}
impl CppLess for MyInt {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_1(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for MyInt {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(operator_lt_1(__lhs.clone(), __rhs.clone()))
            && !(operator_lt_1(__rhs.clone(), __lhs.clone()))
    }
}
impl Clone for MyInt {
    fn clone(&self) -> Self {
        let __src: Value<MyInt> = Rc::new(RefCell::new(MyInt {
            value: self.value.clone(),
        }));
        MyInt::copy_from(__src.as_pointer())
    }
}
impl CppMove for MyInt {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
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
    let mut N: i32 = 1048576;
    let v: Value<Vec<MyInt>> = Rc::new(RefCell::new(Vec::new()));
    if (N as usize) as usize > (*v.borrow()).capacity() as usize {
        let len_0 = (*v.borrow()).len();
        (*v.borrow_mut()).reserve_exact((N as usize) as usize - len_0 as usize);
    };
    let g : Value<std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253_ > = Rc::new(RefCell::new(<std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253_  >::default() )) ;
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < N) {
        {
            let __init = MyInt::new({ (*i.borrow()) });
            v.as_pointer()
                .with_mut(|__v: &mut Vec<MyInt>| __v.push(__init));
            v.as_pointer().decay().to_last()
        };
        (*i.borrow_mut()).prefix_inc();
    }
    let mut logn: i32 = 10;
    'loop_: while (logn <= 20) {
        let n: Value<i32> = Rc::new(RefCell::new((1 << logn)));
        let first: Value<RandomAccessIterator<MyInt>> =
            Rc::new(RefCell::new(CppRange::begin(&v.decay())));
        let last: Value<RandomAccessIterator<MyInt>> = Rc::new(RefCell::new(
            CppRange::begin(&v.decay()).iter_add(({ ((*n.borrow()) as isize) })),
        ));
        let mut debug_elements: i32 = {
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(100));
            (if CppLess::less(&n.as_pointer(), &__tmp_0.as_pointer()) {
                n.as_pointer()
            } else {
                __tmp_0.as_pointer()
            }
            .read())
        };
        let mut debug_comparisons: i32 = ((2 * (debug_elements + 1)) * debug_elements);
        &(debug_comparisons);
        ({
            shuffle_2(
                (*first.borrow()).clone(),
                (*last.borrow()).clone(),
                g.as_pointer(),
            )
        });
        ({ make_heap_3((*first.borrow()).clone(), (*last.borrow()).clone()) });
        stats_0.with(|rc| *rc.borrow_mut() = <Stats>::default());
        ({ sort_heap_4((*first.borrow()).clone(), (*last.borrow()).clone()) });
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        logn.prefix_inc();
    }
    return 0;
}
pub trait MyIntImpl {
    fn copy_assign(&self, other: Ptr<MyInt>) -> Ptr<MyInt>;
    fn move_assign(&self, other: Ptr<MyInt>) -> Ptr<MyInt>;
}
impl MyIntImpl for Ptr<MyInt> {
    fn copy_assign(&self, other: Ptr<MyInt>) -> Ptr<MyInt> {
        field!((*self), value).write({ other.with(|__s| __s.value) });
        (*stats_0.with(Value::clone).borrow_mut())
            .copied
            .prefix_inc();
        return (*self).clone();
    }
    fn move_assign(&self, other: Ptr<MyInt>) -> Ptr<MyInt> {
        field!((*self), value).write({ other.with(|__s| __s.value) });
        (*stats_0.with(Value::clone).borrow_mut())
            .moved
            .prefix_inc();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = stats_0.with(|_| ());
}
