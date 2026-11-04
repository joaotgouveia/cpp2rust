extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct foo {
    #[offset(0)]
    val_: i32,
}
impl foo {
    pub fn new() -> Self {
        Self { val_: 3 }
    }
}
impl CppMove for foo {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for foo {
    fn default() -> Self {
        { foo::new() }
    }
}
pub fn test_0() -> bool {
    {
        let mut p1: Option<Value<Box<[i32]>>> = Some(Rc::new(RefCell::new(
            (0..5_usize).map(|_| 0_i32).collect::<Box<[_]>>(),
        )));
        let mut i: i32 = 0;
        'loop_: while (i < 5) {
            assert!(
                (((p1.as_pointer() as Ptr<i32>)
                    .offset((i as usize) as isize)
                    .read())
                    == 0)
            );
            i.prefix_inc();
        }
    }
    {
        let mut p2: Option<Value<Box<[Vec<i8>]>>> = Some(Rc::new(RefCell::new(
            (0..5_usize)
                .map(|_| vec![i8::default()])
                .collect::<Box<[_]>>(),
        )));
        let mut i: i32 = 0;
        'loop_: while (i < 5) {
            assert!(
                ({
                    (((*(p2.as_pointer() as Ptr<Vec<i8>>)
                        .offset((i as usize) as isize)
                        .upgrade()
                        .deref())
                    .len()
                        - 1)
                        == 0_usize)
                })
            );
            i.prefix_inc();
        }
    }
    {
        let mut p3: Option<Value<Box<[foo]>>> = Some(Rc::new(RefCell::new(
            (0..7_usize).map(|_| <foo>::default()).collect::<Box<[_]>>(),
        )));
        let mut i: i32 = 0;
        'loop_: while (i < 7) {
            assert!(
                (({ fooImpl::get(&(p3.as_pointer() as Ptr<foo>).offset((i as usize) as isize),) })
                    == 3)
            );
            i.prefix_inc();
        }
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait fooImpl {
    fn get(&self) -> i32;
}
impl fooImpl for Ptr<foo> {
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.val_);
    }
}
pub fn __cpp2rust_init_globals() {}
