extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(32)]
pub struct Checker {
    #[offset(0)]
    #[byte_size(8)]
    pub cerr_mem_dump: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub cin_mem_dump: Ptr<()>,
    #[offset(16)]
    #[byte_size(8)]
    pub cout_mem_dump: Ptr<()>,
    #[offset(24)]
    #[byte_size(8)]
    pub clog_mem_dump: Ptr<()>,
}
impl Checker {
    pub fn new() -> Self {
        let __this: Value<Checker> = Rc::new(RefCell::new(Self {
            cerr_mem_dump: Ptr::alloc_array(
                (0..160usize)
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            ),
            cin_mem_dump: Ptr::alloc_array(
                (0..168usize)
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            ),
            cout_mem_dump: Ptr::alloc_array(
                (0..160usize)
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            ),
            clog_mem_dump: Ptr::alloc_array(
                (0..160usize)
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            ),
        }));
        let this: Ptr<Checker> = __this.as_pointer();
        ({
            let ___src: () =
                ((cerr_0.with(|v| v.as_pointer())).reinterpret_cast::<()>() as Ptr<()>).to_any();
            let ___n: () = 160usize;
            memcpy_1(
                (this.with(|__s| __s.cerr_mem_dump.clone()) as Ptr<()>).to_any(),
                ___src,
                ___n,
            )
        });
        ({
            let ___src: () =
                ((cin_2.with(|v| v.as_pointer())).reinterpret_cast::<()>() as Ptr<()>).to_any();
            let ___n: () = 168usize;
            memcpy_1(
                (this.with(|__s| __s.cin_mem_dump.clone()) as Ptr<()>).to_any(),
                ___src,
                ___n,
            )
        });
        ({
            let ___src: () =
                ((cout_3.with(|v| v.as_pointer())).reinterpret_cast::<()>() as Ptr<()>).to_any();
            let ___n: () = 160usize;
            memcpy_1(
                (this.with(|__s| __s.cout_mem_dump.clone()) as Ptr<()>).to_any(),
                ___src,
                ___n,
            )
        });
        ({
            let ___src: () =
                ((clog_4.with(|v| v.as_pointer())).reinterpret_cast::<()>() as Ptr<()>).to_any();
            let ___n: () = 160usize;
            memcpy_1(
                (this.with(|__s| __s.clog_mem_dump.clone()) as Ptr<()>).to_any(),
                ___src,
                ___n,
            )
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Checker {
    fn default() -> Self {
        { Checker::new() }
    }
}
thread_local!(
    pub static check_5: Value<Checker> = Rc::new(RefCell::new(Checker::new()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(32)]
pub struct WideChecker {
    #[offset(0)]
    #[byte_size(8)]
    pub wcerr_mem_dump: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub wcin_mem_dump: Ptr<()>,
    #[offset(16)]
    #[byte_size(8)]
    pub wcout_mem_dump: Ptr<()>,
    #[offset(24)]
    #[byte_size(8)]
    pub wclog_mem_dump: Ptr<()>,
}
impl WideChecker {
    pub fn new() -> Self {
        let __this: Value<WideChecker> = Rc::new(RefCell::new(Self {
            wcerr_mem_dump: Ptr::alloc_array(
                (0..160usize)
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            ),
            wcin_mem_dump: Ptr::alloc_array(
                (0..168usize)
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            ),
            wcout_mem_dump: Ptr::alloc_array(
                (0..160usize)
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            ),
            wclog_mem_dump: Ptr::alloc_array(
                (0..160usize)
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            ),
        }));
        let this: Ptr<WideChecker> = __this.as_pointer();
        ({
            let ___src: () =
                ((wcerr_6.with(|v| v.as_pointer())).reinterpret_cast::<()>() as Ptr<()>).to_any();
            let ___n: () = 160usize;
            memcpy_1(
                (this.with(|__s| __s.wcerr_mem_dump.clone()) as Ptr<()>).to_any(),
                ___src,
                ___n,
            )
        });
        ({
            let ___src: () =
                ((wcin_7.with(|v| v.as_pointer())).reinterpret_cast::<()>() as Ptr<()>).to_any();
            let ___n: () = 168usize;
            memcpy_1(
                (this.with(|__s| __s.wcin_mem_dump.clone()) as Ptr<()>).to_any(),
                ___src,
                ___n,
            )
        });
        ({
            let ___src: () =
                ((wcout_8.with(|v| v.as_pointer())).reinterpret_cast::<()>() as Ptr<()>).to_any();
            let ___n: () = 160usize;
            memcpy_1(
                (this.with(|__s| __s.wcout_mem_dump.clone()) as Ptr<()>).to_any(),
                ___src,
                ___n,
            )
        });
        ({
            let ___src: () =
                ((wclog_9.with(|v| v.as_pointer())).reinterpret_cast::<()>() as Ptr<()>).to_any();
            let ___n: () = 160usize;
            memcpy_1(
                (this.with(|__s| __s.wclog_mem_dump.clone()) as Ptr<()>).to_any(),
                ___src,
                ___n,
            )
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for WideChecker {
    fn default() -> Self {
        { WideChecker::new() }
    }
}
thread_local!(
    pub static wide_check_10: Value<WideChecker> = Rc::new(RefCell::new(WideChecker::new()));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait CheckerImpl {
    fn destructor(&self);
}
impl CheckerImpl for Ptr<Checker> {
    fn destructor(&self) {
        (*self).with(|__s| __s.cerr_mem_dump.clone()).delete();
        (*self).with(|__s| __s.cin_mem_dump.clone()).delete();
        (*self).with(|__s| __s.cout_mem_dump.clone()).delete();
        (*self).with(|__s| __s.clog_mem_dump.clone()).delete();
    }
}
pub trait WideCheckerImpl {
    fn destructor(&self);
}
impl WideCheckerImpl for Ptr<WideChecker> {
    fn destructor(&self) {
        (*self).with(|__s| __s.wcerr_mem_dump.clone()).delete();
        (*self).with(|__s| __s.wcin_mem_dump.clone()).delete();
        (*self).with(|__s| __s.wcout_mem_dump.clone()).delete();
        (*self).with(|__s| __s.wclog_mem_dump.clone()).delete();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = check_5.with(|_| ());
    let _ = wide_check_10.with(|_| ());
}
