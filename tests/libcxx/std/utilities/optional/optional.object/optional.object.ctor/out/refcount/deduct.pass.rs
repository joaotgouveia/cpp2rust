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
pub struct A {}
impl CppMove for A {
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
        let opt: Value<std_optional_int_> = Rc::new(RefCell::new(5));
        const _: () = assert!(true, "");;
        assert!(({ ({ (*opt.borrow())() }) }));
        assert!(({ (((*opt.borrow()).read()) == 5) }));
    }
    {
        let opt: Value<std_optional_A_> = Rc::new(RefCell::new(<A>::default()));
        const _: () = assert!(true, "");;
        assert!(({ ({ (*opt.borrow())() }) }));
    }
    {
        let __tmp_0: Value<i32> = Rc::new(RefCell::new(5));
        let source: Ptr<i32> = __tmp_0.as_pointer();
        let opt: Value<std_optional_int_> =
            Rc::new(RefCell::new(std_optional_int_::new_1({ (source).clone() })));
        const _: () = assert!(true, "");;
        assert!(({ ({ (*opt.borrow())() }) }));
        assert!(({ (((*opt.borrow()).read()) == 5) }));
    }
    {
        let source: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
        let opt: Value<std_optional_const_int_ptr_> =
            Rc::new(RefCell::new(std_optional_const_int_ptr_::new_1({
                source.as_pointer()
            })));
        const _: () = assert!(true, "");;
        assert!(({ ({ (*opt.borrow())() }) }));
        assert!(({ ((*opt.borrow()).read()).is_null() }));
    }
    {
        let source: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
        let opt: Value<std_optional_int_ptr_> =
            Rc::new(RefCell::new(std_optional_int_ptr_::new_1({
                source.as_pointer()
            })));
        const _: () = assert!(true, "");;
        assert!(({ ({ (*opt.borrow())() }) }));
        assert!(({ ((elem!(((*opt.borrow()).read()), 0).read()) == 1) }));
    }
    {
        let source: Value<std_optional_char_> = Rc::new(RefCell::new(('A' as i8)));
        let opt: Value<std_optional_char_> = Rc::new(RefCell::new((*source.borrow()).clone()));
        const _: () = assert!(true, "");;
        assert!(({ ((({ (*opt.borrow())() }) as i32) == (({ (*source.borrow())() }) as i32)) }));
        assert!(({ ((((*opt.borrow()).read()) as i32) == (((*source.borrow()).read()) as i32)) }));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
