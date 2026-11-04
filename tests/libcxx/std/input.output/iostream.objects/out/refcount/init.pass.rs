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
    pub cerr_mem_dump: Ptr<i8>,
    #[offset(8)]
    #[byte_size(8)]
    pub cin_mem_dump: Ptr<i8>,
    #[offset(16)]
    #[byte_size(8)]
    pub cout_mem_dump: Ptr<i8>,
    #[offset(24)]
    #[byte_size(8)]
    pub clog_mem_dump: Ptr<i8>,
}
impl Checker {
    pub fn new() -> Self {
        let __this: Value<Checker> = Rc::new(RefCell::new(Self {
            cerr_mem_dump: Ptr::alloc_array((0..272usize).map(|_| 0_i8).collect::<Box<[i8]>>()),
            cin_mem_dump: Ptr::alloc_array((0..280usize).map(|_| 0_i8).collect::<Box<[i8]>>()),
            cout_mem_dump: Ptr::alloc_array((0..272usize).map(|_| 0_i8).collect::<Box<[i8]>>()),
            clog_mem_dump: Ptr::alloc_array((0..272usize).map(|_| 0_i8).collect::<Box<[i8]>>()),
        }));
        let this: Ptr<Checker> = __this.as_pointer();
        ({
            let ___src: AnyPtr =
                ((cerr_0.with(|v| v.as_pointer())).reinterpret_cast::<i8>() as Ptr<i8>).to_any();
            let ___n: u64 = (272usize as u64);
            memcpy_1(
                (this.with(|__s| __s.cerr_mem_dump.clone()) as Ptr<i8>).to_any(),
                ___src,
                ___n,
            )
        });
        ({
            let ___src: AnyPtr =
                ((cin_2.with(|v| v.as_pointer())).reinterpret_cast::<i8>() as Ptr<i8>).to_any();
            let ___n: u64 = (280usize as u64);
            memcpy_1(
                (this.with(|__s| __s.cin_mem_dump.clone()) as Ptr<i8>).to_any(),
                ___src,
                ___n,
            )
        });
        ({
            let ___src: AnyPtr =
                ((cout_3.with(|v| v.as_pointer())).reinterpret_cast::<i8>() as Ptr<i8>).to_any();
            let ___n: u64 = (272usize as u64);
            memcpy_1(
                (this.with(|__s| __s.cout_mem_dump.clone()) as Ptr<i8>).to_any(),
                ___src,
                ___n,
            )
        });
        ({
            let ___src: AnyPtr =
                ((clog_4.with(|v| v.as_pointer())).reinterpret_cast::<i8>() as Ptr<i8>).to_any();
            let ___n: u64 = (272usize as u64);
            memcpy_1(
                (this.with(|__s| __s.clog_mem_dump.clone()) as Ptr<i8>).to_any(),
                ___src,
                ___n,
            )
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppMove for Checker {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
    pub wcerr_mem_dump: Ptr<i8>,
    #[offset(8)]
    #[byte_size(8)]
    pub wcin_mem_dump: Ptr<i8>,
    #[offset(16)]
    #[byte_size(8)]
    pub wcout_mem_dump: Ptr<i8>,
    #[offset(24)]
    #[byte_size(8)]
    pub wclog_mem_dump: Ptr<i8>,
}
impl WideChecker {
    pub fn new() -> Self {
        let __this: Value<WideChecker> = Rc::new(RefCell::new(Self {
            wcerr_mem_dump: Ptr::alloc_array((0..272usize).map(|_| 0_i8).collect::<Box<[i8]>>()),
            wcin_mem_dump: Ptr::alloc_array((0..280usize).map(|_| 0_i8).collect::<Box<[i8]>>()),
            wcout_mem_dump: Ptr::alloc_array((0..272usize).map(|_| 0_i8).collect::<Box<[i8]>>()),
            wclog_mem_dump: Ptr::alloc_array((0..272usize).map(|_| 0_i8).collect::<Box<[i8]>>()),
        }));
        let this: Ptr<WideChecker> = __this.as_pointer();
        ({
            let ___src: AnyPtr =
                ((wcerr_6.with(|v| v.as_pointer())).reinterpret_cast::<i8>() as Ptr<i8>).to_any();
            let ___n: u64 = (272usize as u64);
            memcpy_1(
                (this.with(|__s| __s.wcerr_mem_dump.clone()) as Ptr<i8>).to_any(),
                ___src,
                ___n,
            )
        });
        ({
            let ___src: AnyPtr =
                ((wcin_7.with(|v| v.as_pointer())).reinterpret_cast::<i8>() as Ptr<i8>).to_any();
            let ___n: u64 = (280usize as u64);
            memcpy_1(
                (this.with(|__s| __s.wcin_mem_dump.clone()) as Ptr<i8>).to_any(),
                ___src,
                ___n,
            )
        });
        ({
            let ___src: AnyPtr =
                ((wcout_8.with(|v| v.as_pointer())).reinterpret_cast::<i8>() as Ptr<i8>).to_any();
            let ___n: u64 = (272usize as u64);
            memcpy_1(
                (this.with(|__s| __s.wcout_mem_dump.clone()) as Ptr<i8>).to_any(),
                ___src,
                ___n,
            )
        });
        ({
            let ___src: AnyPtr =
                ((wclog_9.with(|v| v.as_pointer())).reinterpret_cast::<i8>() as Ptr<i8>).to_any();
            let ___n: u64 = (272usize as u64);
            memcpy_1(
                (this.with(|__s| __s.wclog_mem_dump.clone()) as Ptr<i8>).to_any(),
                ___src,
                ___n,
            )
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppMove for WideChecker {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({
        cpp2rust_assert_fail_11(
            (({
                let ___s2: AnyPtr = ((cerr_0.with(|v| v.as_pointer())).reinterpret_cast::<i8>()
                    as Ptr<i8>)
                    .to_any();
                let ___n: u64 = (272usize as u64);
                memcmp_12(
                    ({ (*check_5.with(Value::clone).borrow()).cerr_mem_dump.clone() } as Ptr<i8>)
                        .to_any(),
                    ___s2,
                    ___n,
                )
            }) == 0),
        )
    });
    ({
        cpp2rust_assert_fail_11(
            (({
                let ___s2: AnyPtr =
                    ((cin_2.with(|v| v.as_pointer())).reinterpret_cast::<i8>() as Ptr<i8>).to_any();
                let ___n: u64 = (280usize as u64);
                memcmp_12(
                    ({ (*check_5.with(Value::clone).borrow()).cin_mem_dump.clone() } as Ptr<i8>)
                        .to_any(),
                    ___s2,
                    ___n,
                )
            }) == 0),
        )
    });
    ({
        cpp2rust_assert_fail_11(
            (({
                let ___s2: AnyPtr = ((cout_3.with(|v| v.as_pointer())).reinterpret_cast::<i8>()
                    as Ptr<i8>)
                    .to_any();
                let ___n: u64 = (272usize as u64);
                memcmp_12(
                    ({ (*check_5.with(Value::clone).borrow()).cout_mem_dump.clone() } as Ptr<i8>)
                        .to_any(),
                    ___s2,
                    ___n,
                )
            }) == 0),
        )
    });
    ({
        cpp2rust_assert_fail_11(
            (({
                let ___s2: AnyPtr = ((clog_4.with(|v| v.as_pointer())).reinterpret_cast::<i8>()
                    as Ptr<i8>)
                    .to_any();
                let ___n: u64 = (272usize as u64);
                memcmp_12(
                    ({ (*check_5.with(Value::clone).borrow()).clog_mem_dump.clone() } as Ptr<i8>)
                        .to_any(),
                    ___s2,
                    ___n,
                )
            }) == 0),
        )
    });
    ({
        cpp2rust_assert_fail_11(
            (({
                let ___s2: AnyPtr = ((wcerr_6.with(|v| v.as_pointer())).reinterpret_cast::<i8>()
                    as Ptr<i8>)
                    .to_any();
                let ___n: u64 = (272usize as u64);
                memcmp_12(
                    ({
                        (*wide_check_10.with(Value::clone).borrow())
                            .wcerr_mem_dump
                            .clone()
                    } as Ptr<i8>)
                        .to_any(),
                    ___s2,
                    ___n,
                )
            }) == 0),
        )
    });
    ({
        cpp2rust_assert_fail_11(
            (({
                let ___s2: AnyPtr = ((wcin_7.with(|v| v.as_pointer())).reinterpret_cast::<i8>()
                    as Ptr<i8>)
                    .to_any();
                let ___n: u64 = (280usize as u64);
                memcmp_12(
                    ({
                        (*wide_check_10.with(Value::clone).borrow())
                            .wcin_mem_dump
                            .clone()
                    } as Ptr<i8>)
                        .to_any(),
                    ___s2,
                    ___n,
                )
            }) == 0),
        )
    });
    ({
        cpp2rust_assert_fail_11(
            (({
                let ___s2: AnyPtr = ((wcout_8.with(|v| v.as_pointer())).reinterpret_cast::<i8>()
                    as Ptr<i8>)
                    .to_any();
                let ___n: u64 = (272usize as u64);
                memcmp_12(
                    ({
                        (*wide_check_10.with(Value::clone).borrow())
                            .wcout_mem_dump
                            .clone()
                    } as Ptr<i8>)
                        .to_any(),
                    ___s2,
                    ___n,
                )
            }) == 0),
        )
    });
    ({
        cpp2rust_assert_fail_11(
            (({
                let ___s2: AnyPtr = ((wclog_9.with(|v| v.as_pointer())).reinterpret_cast::<i8>()
                    as Ptr<i8>)
                    .to_any();
                let ___n: u64 = (272usize as u64);
                memcmp_12(
                    ({
                        (*wide_check_10.with(Value::clone).borrow())
                            .wclog_mem_dump
                            .clone()
                    } as Ptr<i8>)
                        .to_any(),
                    ___s2,
                    ___n,
                )
            }) == 0),
        )
    });
    return 0;
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
