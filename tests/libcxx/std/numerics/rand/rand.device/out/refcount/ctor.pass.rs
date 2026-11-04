extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn check_random_device_valid_0(
    token: Ptr<std_basic_string_char__std_char_traits_char___std_allocator_char__>,
) {
    let r: Value<std_random_device> =
        Rc::new(RefCell::new(std_random_device::new_1({ (token).clone() })));
}
pub fn check_random_device_invalid_1(
    token: Ptr<std_basic_string_char__std_char_traits_char___std_allocator_char__>,
) {
    (&(*token.upgrade().deref()));
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
        let r: Value<std_random_device> = Rc::new(RefCell::new(<std_random_device>::default()));
        &(*r.borrow_mut());
    }
    {
        ({
            let _token: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
                Rc::new(RefCell::new(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"/dev/urandom") },
                        None,
                    ),
                ));
            check_random_device_valid_0(_token.as_pointer())
        });
        ({
            let _token: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
                Rc::new(RefCell::new(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"/dev/random") },
                        None,
                    ),
                ));
            check_random_device_invalid_1(_token.as_pointer())
        });
        ({
            let _token: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
                Rc::new(RefCell::new(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"/dev/null") },
                        None,
                    ),
                ));
            check_random_device_invalid_1(_token.as_pointer())
        });
        ({
            let _token: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
                Rc::new(RefCell::new(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"/dev/nonexistent") },
                        None,
                    ),
                ));
            check_random_device_invalid_1(_token.as_pointer())
        });
        ({
            let _token: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
                Rc::new(RefCell::new(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"wrong file") },
                        None,
                    ),
                ));
            check_random_device_invalid_1(_token.as_pointer())
        });
    }
    {
        let mut ec: i32 = 0_i32;
        ec = ({ close_2(0) });
        ({ cpp2rust_assert_fail_3(!(ec != 0)) });
        ec = ({ close_2(1) });
        ({ cpp2rust_assert_fail_3(!(ec != 0)) });
        ec = ({ close_2(2) });
        ({ cpp2rust_assert_fail_3(!(ec != 0)) });
        let r: Value<std_random_device> = Rc::new(RefCell::new(<std_random_device>::default()));
    }
    const _: () = assert!(true, "test_convertible<std::random_device>()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
