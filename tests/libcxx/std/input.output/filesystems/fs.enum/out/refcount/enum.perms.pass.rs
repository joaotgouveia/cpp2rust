extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn ME_0(mut val: i32) -> std_filesystem_perms {
    return ((val) as std_filesystem_perms);
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
    assert!(
        ({
            check_bitmask_type_std_filesystem_perms__std_filesystem_perms_group_all__std_filesystem_perms_owner_all__unsigned_int__56__448__0__std_filesystem_perms_none_::check ( )
        })
    );
    const _: () = assert!(
        true,
        "E::none         == ME(0) &&\n\n        E::owner_read   == ME(0400) &&\n        E::owner_write  == ME(0200) &&\n        E::owner_exec   == ME(0100) &&\n        E::owner_all    == ME(0700) &&\n\n        E::group_read   == ME(040) &&\n        E::group_write  == ME(020) &&\n        E::group_exec   == ME(010) &&\n        E::group_all    == ME(070) &&\n\n        E::others_read  == ME(04) &&\n        E::others_write == ME(02) &&\n        E::others_exec  == ME(01) &&\n        E::others_all   == ME(07) &&\n        E::all          == ME(0777) &&\n        E::set_uid      == ME(04000) &&\n        E::set_gid      == ME(02000) &&\n        E::sticky_bit   == ME(01000) &&\n        E::mask         == ME(07777) &&\n        E::unknown      == ME(0xFFFF)"
    );;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
