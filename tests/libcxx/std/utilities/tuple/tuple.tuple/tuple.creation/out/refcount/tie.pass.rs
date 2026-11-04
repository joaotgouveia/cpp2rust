extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_tie_0() -> bool {
    {
        let i: Value<i32> = Rc::new(RefCell::new(42));
        let f: Value<f64> = Rc::new(RefCell::new(1.1_f64));
        let res: Value<std_tuple_int_ref__const_std__Swallow_assign_ref__double_ref_> =
            Rc::new(RefCell::new(
                ({
                    tie_1(
                        i.as_pointer(),
                        ignore_2.with(|v| v.as_pointer()),
                        f.as_pointer(),
                    )
                }),
            ));
        const _: () = assert!(true, "std::is_same<ExpectT, decltype(res)>::value");;
        assert!((({ get_3(res.as_pointer(),) }) == (i.as_pointer())));
        assert!((({ get_4(res.as_pointer(),) }) == (ignore_2.with(|v| v.as_pointer()))));
        assert!((({ get_5(res.as_pointer(),) }) == (f.as_pointer())));
        (*res.borrow_mut()) = ({
            let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
            let ___args_1: Value<AnyPtr> = Rc::new(RefCell::new(Default::default()));
            let ___args_2: Value<f64> = Rc::new(RefCell::new(-1_f64));
            make_tuple_6(
                ___args_0.as_pointer(),
                ___args_1.as_pointer(),
                ___args_2.as_pointer(),
            )
        });
        assert!(({ ((*i.borrow()) == 101) }));
        assert!(({ ((*f.borrow()) == -1_f64) }));
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
    ({ test_tie_0() });
    const _: () = assert!(true, "test_tie()");;
    {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(vec![i8::default()]));
        ({
            tie_8(
                i.as_pointer(),
                ignore_2.with(|v| v.as_pointer()),
                (s.as_pointer()).reinterpret_cast::<Vec<i8>>(),
            )
        }) = ({
            let ___args_0: Value<i32> = Rc::new(RefCell::new(42));
            let ___args_1: Value<f64> = Rc::new(RefCell::new(3.14_f64));
            make_tuple_7(
                ___args_0.as_pointer(),
                ___args_1.as_pointer(),
                Ptr::<i8>::from_string_literal(b"C++"),
            )
        });
        assert!(({ ((*i.borrow()) == 42) }));
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"C++").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
