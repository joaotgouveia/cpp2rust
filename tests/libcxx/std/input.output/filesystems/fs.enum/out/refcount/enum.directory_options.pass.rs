extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn ME_0(mut val: i32) -> std_filesystem_directory_options {
    return ((val) as std_filesystem_directory_options);
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
    const _: () = assert!(true, "std::is_enum<E>::value");;
    const _: () = assert!(true, "!std::is_convertible<E, UT>::value");;
    const _: () = assert!(true, "");;
    ({
        cpp2rust_assert_fail_1(
            ({
                check_bitmask_type_std_filesystem_directory_options__std_filesystem_directory_options_follow_directory_symlink__std_filesystem_directory_options_skip_permission_denied__unsigned_char____x01_____x02_____x00___std_filesystem_directory_options_none_::check ( )
            }),
        )
    });
    const _: () = assert!(
        true,
        "E::none                     == ME(0) &&\n        E::follow_directory_symlink == ME(1) &&\n        E::skip_permission_denied   == ME(2)"
    );;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
