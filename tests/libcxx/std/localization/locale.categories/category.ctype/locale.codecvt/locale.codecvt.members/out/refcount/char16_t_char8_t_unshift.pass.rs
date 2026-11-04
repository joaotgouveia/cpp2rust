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
    let f: Ptr<std_codecvt_char16_t__char8_t____mbstate_t_> =
        ({ use_facet_0(({ std_locale::classic() })) });
    let mbs: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
    let to: Value<Box<[u8]>> = Rc::new(RefCell::new((0..3).map(|_| 0_u8).collect::<Box<[u8]>>()));
    let to_next: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
    assert!(
        ({
            ((({
                let ___to: Ptr<u8> = (to.as_pointer() as Ptr<u8>);
                let ___to_end: Ptr<u8> = (to.as_pointer() as Ptr<u8>).offset((3) as isize);
                ((*f.upgrade().deref())
                    as std___codecvt_abstract_base_char16_t__char8_t____mbstate_t_)
                    .unshift(mbs.as_pointer(), ___to, ___to_end, to_next.as_pointer())
            }) as i32)
                == (std_codecvt_base_result_noconv as i32))
        })
    );
    assert!(({ ({ (*to_next.borrow()).clone() } == { (to.as_pointer() as Ptr::<u8>) }) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
