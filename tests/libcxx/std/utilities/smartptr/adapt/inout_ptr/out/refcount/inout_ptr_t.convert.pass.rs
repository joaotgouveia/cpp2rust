extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

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
        let uPtr: Value<Option<Value<i32>>> =
            Rc::new(RefCell::new(Some(Rc::new(RefCell::new(84)))));
        let ioPtr: Value<std_inout_ptr_t_std_unique_ptr_int__std_default_delete_int____int_ptr_> =
            Rc::new(RefCell::new(
                std_inout_ptr_t_std_unique_ptr_int__std_default_delete_int____int_ptr_::new({
                    uPtr.as_pointer()
                }),
            ));
        const _: () = assert!(true, "noexcept(ioPtr.operator int**())");;
        let mut pPtr: Ptr<Ptr<i32>> = ({ (*ioPtr.borrow())() });
        assert!((((pPtr.read()).read()) == 84));
    }
    {
        let uPtr: Value<Option<Value<i32>>> =
            Rc::new(RefCell::new(Some(Rc::new(RefCell::new(84)))));
        let ioPtr : Value<std_inout_ptr_t_std_unique_ptr_int__std_default_delete_int____int_ptr__std_default_delete_int__ > = Rc::new(RefCell::new(std_inout_ptr_t_std_unique_ptr_int__std_default_delete_int____int_ptr__std_default_delete_int__ :: new ( { uPtr .as_pointer()  } , { <std_default_delete_int_  >::default()  } , )  )) ;
        const _: () = assert!(true, "noexcept(ioPtr.operator int**())");;
        let mut pPtr: Ptr<Ptr<i32>> = ({ (*ioPtr.borrow())() });
        assert!((((pPtr.read()).read()) == 84));
    }
    {
        let uPtr: Value<Option<Value<i32>>> =
            Rc::new(RefCell::new(Some(Rc::new(RefCell::new(84)))));
        let ioPtr: Value<std_inout_ptr_t_std_unique_ptr_int__std_default_delete_int____void_ptr_> =
            Rc::new(RefCell::new(
                std_inout_ptr_t_std_unique_ptr_int__std_default_delete_int____void_ptr_::new({
                    uPtr.as_pointer()
                }),
            ));
        const _: () = assert!(true, "noexcept(ioPtr.operator void**())");;
        let mut pPtr: Ptr<AnyPtr> = ({ (*ioPtr.borrow()).to_pmutAnyPtr_4() });
        assert!((((pPtr.reinterpret_cast::<Ptr<i32>>().read()).read()) == 84));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
