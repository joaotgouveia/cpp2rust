extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct StaticInit {
    #[offset(0)]
    #[byte_size(8)]
    pub ec: Ptr<std_error_category>,
}
impl CppMove for StaticInit {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static foo_0: Value<StaticInit> = Rc::new(RefCell::new(<StaticInit>::default()));
);

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
        let e_cat1: Ptr<std_error_category> = ({ system_category_1() });
        let e_cond: Value<std_error_condition> = Rc::new(RefCell::new(
            ({ (*e_cat1.upgrade().deref()).default_error_condition(5) }),
        ));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(
            ({
                ({
                    let ___cond: Ptr<std_error_condition> = e_cond.as_pointer();
                    (*e_cat1.upgrade().deref()).equivalent_9(5, ___cond)
                })
            })
        );
        (*e_cond.borrow_mut()) = ({ (*e_cat1.upgrade().deref()).default_error_condition(5000) });
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(
            ({
                ({
                    let ___cond: Ptr<std_error_condition> = e_cond.as_pointer();
                    (*e_cat1.upgrade().deref()).equivalent_9(5000, ___cond)
                })
            })
        );
    }
    {
        libcc2rs::cpp2rust_errno().write(libc::E2BIG);
        let e_cat1: Ptr<std_error_category> = ({ system_category_1() });
        let mut msg: Vec<i8> = ({ (*e_cat1.upgrade().deref()).message(-1_i32) });
        let mut is_newlib: bool = false;
        &(is_newlib);
        const _: () = assert!(true, "");;
        assert!(((libcc2rs::cpp2rust_errno().read()) == libc::E2BIG));
    }
    {
        (*foo_0.with(Value::clone).borrow_mut()).ec = ({ system_category_1() });
        let mut m: Vec<i8> = {
            let mut __bytes = ({
                ({
                    (*{ (*foo_0.with(Value::clone).borrow()).ec.clone() }
                        .upgrade()
                        .deref())
                    .name()
                })
            })
            .to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        };
        assert!(
            Ptr::<i8>::from_string_literal(b"system")
                .with_c_str(|__s| m[..m.len().saturating_sub(1)] == *__s)
        );
    }
    return 0;
}
pub trait StaticInitImpl {
    fn destructor(&self);
}
impl StaticInitImpl for Ptr<StaticInit> {
    fn destructor(&self) {
        let mut str: Vec<i8> = {
            let mut __bytes =
                ({ ({ (*(*self).with(|__s| __s.ec.clone()).upgrade().deref()).name() }) })
                    .to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        };
        assert!(
            Ptr::<i8>::from_string_literal(b"system")
                .with_c_str(|__s| str[..str.len().saturating_sub(1)] == *__s)
        );
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = foo_0.with(|_| ());
}
