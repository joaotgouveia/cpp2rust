extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_eprintf_0(
    mut fmt: Ptr<()>,
    args_1: Ptr<Ptr<()>>,
    args_2: Ptr<Ptr<()>>,
    args_3: Ptr<Ptr<()>>,
    args_4: Ptr<()>,
    args_5: Ptr<Ptr<()>>,
) {
    ({
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            (fmt).clone() as *const libc::c_char,
            (args_1.read()),
            (args_2.read()),
            (args_3.read()),
            (args_4.read()),
            (args_5.read()),
        )
    });
}
pub fn test_log_1(condition: Ptr<()>, file: Ptr<()>, line: (), message: Ptr<()>) {
    let condition: Value<Ptr<()>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<()>> = Rc::new(RefCell::new(file));
    let line: Value<()> = Rc::new(RefCell::new(line));
    let message: Value<Ptr<()>> = Rc::new(RefCell::new(message));
    let msg: Value<Ptr<()>> = Rc::new(RefCell::new(if !(*condition.borrow()).is_null() {
        Ptr::<()>::from_string_literal(b"Assertion failure: ")
    } else {
        Ptr::<()>::from_string_literal(b"Unconditional failure:")
    }));
    ({
        test_eprintf_0(
            Ptr::<()>::from_string_literal(b"%s%s %s %d\n%s"),
            msg.as_pointer(),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
            message.as_pointer(),
        )
    });
}
pub fn test_fail_2(mut file: Ptr<()>, mut line: (), arg: Ptr<()>) {
    ({
        test_log_1(
            Ptr::<()>::from_string_literal(b""),
            (file).clone(),
            line,
            (arg).clone(),
        )
    });
    unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(80)]
pub struct MultiStringType {
    #[offset(0)]
    #[byte_size(8)]
    pub w_: Ptr<()>,
    #[offset(8)]
    pub wn_: (),
    #[offset(16)]
    #[byte_size(8)]
    pub u8_: Ptr<()>,
    #[offset(24)]
    pub u8n_: (),
    #[offset(32)]
    #[byte_size(8)]
    pub u16_: Ptr<()>,
    #[offset(40)]
    pub u16n_: (),
    #[offset(48)]
    #[byte_size(8)]
    pub u32_: Ptr<()>,
    #[offset(56)]
    pub u32n_: (),
    #[offset(64)]
    #[byte_size(8)]
    pub s_: Ptr<()>,
    #[offset(72)]
    pub sn_: (),
}
impl MultiStringType {
    pub fn new(
        mut w: Ptr<()>,
        mut wn: (),
        mut u8: Ptr<()>,
        mut u8n: (),
        mut u16: Ptr<()>,
        mut u16n: (),
        mut u32: Ptr<()>,
        mut u32n: (),
        mut s: Ptr<()>,
        mut sn: (),
    ) -> Self {
        Self {
            w_: (w).clone(),
            wn_: wn,
            u8_: (u8).clone(),
            u8n_: u8n,
            u16_: (u16).clone(),
            u16n_: u16n,
            u32_: (u32).clone(),
            u32n_: u32n,
            s_: (s).clone(),
            sn_: sn,
        }
    }
}
pub fn format_string_imp_3(msg: Ptr<()>, __args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let msg: Value<Ptr<()>> = Rc::new(RefCell::new(msg));
    let args: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*args.borrow_mut()) = __args.as_pointer();
    let args_guard: Value<GuardVAList_4> =
        Rc::new(RefCell::new(GuardVAList_4::new({ args.as_pointer() })));
    let _dtor_args_guard = ScopedDestructor::new(&args_guard, |__p| __p.destructor());
    let args_cp: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*args_cp.borrow_mut()) = (*args.borrow_mut()).clone();
    let args_copy_guard: Value<GuardVAList_4> =
        Rc::new(RefCell::new(GuardVAList_4::new({ args_cp.as_pointer() })));
    let _dtor_args_copy_guard = ScopedDestructor::new(&args_copy_guard, |__p| __p.destructor());
    let local_buff: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut size: () = unimplemented!();
    let mut ret: () = ({
        vsnprintf_5(
            unimplemented!(),
            size,
            (*msg.borrow()).clone(),
            (*args_cp.borrow()).clone(),
        )
    });
    ({ GuardVAList_4Impl::clear(&args_copy_guard.as_pointer()) });
    if (ret == 0_()) {
        return unimplemented!();
    }
    if (ret < size) {
        return unimplemented!();
    }
    size = (ret).wrapping_add(1_());
    let mut buff_ptr: () = unimplemented!();
    ret = ({
        vsnprintf_5(
            unimplemented!(),
            size,
            (*msg.borrow()).clone(),
            (*args.borrow()).clone(),
        )
    });
    return unimplemented!();
}
pub fn unwrap_6(s: Ptr<()>) -> Ptr<()> {
    return unimplemented!();
}
pub fn unwrap_7(a: Ptr<Ptr<()>>) -> Ptr<Ptr<()>> {
    const _: () = assert!(true, "!std::is_class<Arg>::value");;
    return (a).clone();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(16)]
pub struct GuardVAList_4 {
    #[offset(0)]
    #[byte_size(8)]
    pub xtarget: Ptr<Ptr<VaArg>>,
    #[offset(8)]
    pub active: (),
}
impl GuardVAList_4 {
    pub fn new(val: Ptr<Ptr<VaArg>>) -> Self {
        Self {
            xtarget: (val).clone(),
            active: true,
        }
    }
}
pub fn format_string_8(
    mut fmt: Ptr<()>,
    args_1: Ptr<Ptr<()>>,
    args_2: Ptr<()>,
    args_3: Ptr<()>,
) -> () {
    return ({
        format_string_imp_3(
            fmt,
            Box::new([
                (({ unwrap_7((args_1).clone()) }).read()).into(),
                ({ unwrap_6((args_2).clone()) }).into(),
                ({ unwrap_6((args_3).clone()) }).into(),
            ]),
        )
    });
}
pub fn format_string_9(
    mut fmt: Ptr<()>,
    args_1: Ptr<Ptr<()>>,
    args_2: Ptr<()>,
    args_3: Ptr<()>,
    args_4: Ptr<Ptr<()>>,
) -> () {
    return ({
        format_string_imp_3(
            fmt,
            Box::new([
                (({ unwrap_7((args_1).clone()) }).read()).into(),
                ({ unwrap_6((args_2).clone()) }).into(),
                ({ unwrap_6((args_3).clone()) }).into(),
                (({ unwrap_7((args_4).clone()) }).read()).into(),
            ]),
        )
    });
}
pub fn format_string_10(
    mut fmt: Ptr<()>,
    args_1: Ptr<Ptr<()>>,
    args_2: Ptr<()>,
    args_3: Ptr<()>,
    args_4: Ptr<Ptr<()>>,
    args_5: Ptr<Ptr<()>>,
) -> () {
    return ({
        format_string_imp_3(
            fmt,
            Box::new([
                (({ unwrap_7((args_1).clone()) }).read()).into(),
                ({ unwrap_6((args_2).clone()) }).into(),
                ({ unwrap_6((args_3).clone()) }).into(),
                (({ unwrap_7((args_4).clone()) }).read()).into(),
                (({ unwrap_7((args_5).clone()) }).read()).into(),
            ]),
        )
    });
}
pub fn symlink_11(mut oldname: Ptr<()>, mut newname: Ptr<()>, mut is_dir: ()) -> () {
    &(is_dir);
    return ({ symlink_12(oldname, newname) });
}
pub fn space_13(mut path: (), capacity: Ptr<u64>, free: Ptr<u64>, avail: Ptr<u64>) -> () {
    let expect: Value<statvfs> = Rc::new(RefCell::new(<statvfs>::default()));
    if (({ statvfs_14(unimplemented!(), (expect.as_pointer())) }) == -1_()) {
        return false;
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let do_mult: Value<FnPtr<fn(u64) -> u64>> = Rc::new(RefCell::new(lambda!(
        {
            let expect: Ptr<statvfs> = expect.as_pointer();
        },
        |mut val: u64| -> u64 {
            let mut fsize: u64 = (expect.with(|__s| __s.f_frsize) as u64);
            let new_val: Value<u64> = Rc::new(RefCell::new((val).wrapping_mul(fsize)));
            unimplemented!();
            return (*new_val.borrow());
        }
    )));
    let __rhs = ({ (*do_mult.borrow()).call(({ (*expect.borrow()).f_blocks } as u64)) });
    capacity.write(__rhs);
    let __rhs = ({ (*do_mult.borrow()).call(({ (*expect.borrow()).f_bfree } as u64)) });
    free.write(__rhs);
    let __rhs = ({ (*do_mult.borrow()).call(({ (*expect.borrow()).f_bavail } as u64)) });
    avail.write(__rhs);
    return true;
}
pub fn fopen64_15(mut pathname: Ptr<()>, mut mode: Ptr<()>) -> *mut ::libc::FILE {
    return libc::fopen(pathname, mode);
}
pub fn ftruncate64_16(mut fd: (), mut length: ::libc::off_t) -> () {
    return unimplemented!();
}
pub fn getcwd_17() -> () {
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..4096).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    let mut ret: Ptr<()> = ({
        let ___buf: Ptr<()> = (buf.as_pointer() as Ptr<()>);
        let ___size: () = ::std::mem::size_of::<[(); 4096]>();
        getcwd_18(___buf, ___size)
    });
    unimplemented!();
    return unimplemented!();
}
pub fn exists_19(path: Ptr<()>) -> () {
    let tmp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    return (unimplemented!() == 0_());
}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(24)]
pub struct scoped_test_env {
    #[offset(0)]
    #[byte_size(24)]
    pub test_root: (),
}
impl scoped_test_env {
    pub fn new() -> Self {
        let __this: scoped_test_env = Self {
            test_root: ({ scoped_test_env::available_cwd_path() }),
        };
        let mut cmd: () = unimplemented!();
        let mut ret: () = ({ system_20(unimplemented!()) });
        unimplemented!();
        unimplemented!();
        __this
    }
    fn available_cwd_path() -> () {
        let cwd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let tmp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut base: () = unimplemented!();
        let mut i: () = unimplemented!();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: while ({
            let _path: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            exists_19(_path.as_pointer())
        }) {
            unimplemented!();
        }
        return unimplemented!();
    }
}
impl Default for scoped_test_env {
    fn default() -> Self {
        { scoped_test_env::new() }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(456)]
pub struct static_test_env {
    #[offset(0)]
    #[byte_size(24)]
    env_: scoped_test_env,
    #[offset(24)]
    #[byte_size(24)]
    pub Root: (),
    #[offset(48)]
    #[byte_size(24)]
    pub TestFileList: (),
    #[offset(72)]
    #[byte_size(24)]
    pub TestDirList: (),
    #[offset(96)]
    #[byte_size(24)]
    pub File: (),
    #[offset(120)]
    #[byte_size(24)]
    pub Dir: (),
    #[offset(144)]
    #[byte_size(24)]
    pub Dir2: (),
    #[offset(168)]
    #[byte_size(24)]
    pub Dir3: (),
    #[offset(192)]
    #[byte_size(24)]
    pub SymlinkToFile: (),
    #[offset(216)]
    #[byte_size(24)]
    pub SymlinkToDir: (),
    #[offset(240)]
    #[byte_size(24)]
    pub BadSymlink: (),
    #[offset(264)]
    #[byte_size(24)]
    pub DNE: (),
    #[offset(288)]
    #[byte_size(24)]
    pub EmptyFile: (),
    #[offset(312)]
    #[byte_size(24)]
    pub NonEmptyFile: (),
    #[offset(336)]
    #[byte_size(24)]
    pub CharFile: (),
    #[offset(360)]
    #[byte_size(24)]
    pub DirIterationList: (),
    #[offset(384)]
    #[byte_size(24)]
    pub DirIterationListDepth1: (),
    #[offset(408)]
    #[byte_size(24)]
    pub RecDirIterationList: (),
    #[offset(432)]
    #[byte_size(24)]
    pub RecDirFollowSymlinksIterationList: (),
}
impl static_test_env {
    pub fn new() -> Self {
        let __this: Value<static_test_env> = Rc::new(RefCell::new(Self {
            env_: scoped_test_env::new(),
            Root: unimplemented!(),
            TestFileList: unimplemented!(),
            TestDirList: unimplemented!(),
            File: unimplemented!(),
            Dir: unimplemented!(),
            Dir2: unimplemented!(),
            Dir3: unimplemented!(),
            SymlinkToFile: ({
                let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                static_test_envImpl::makePath(&this, _p.as_pointer())
            }),
            SymlinkToDir: ({
                let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                static_test_envImpl::makePath(&this, _p.as_pointer())
            }),
            BadSymlink: ({
                let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                static_test_envImpl::makePath(&this, _p.as_pointer())
            }),
            DNE: ({
                let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                static_test_envImpl::makePath(&this, _p.as_pointer())
            }),
            EmptyFile: unimplemented!(),
            NonEmptyFile: unimplemented!(),
            CharFile: unimplemented!(),
            DirIterationList: unimplemented!(),
            DirIterationListDepth1: unimplemented!(),
            RecDirIterationList: unimplemented!(),
            RecDirFollowSymlinksIterationList: unimplemented!(),
        }));
        let this: Ptr<static_test_env> = __this.as_pointer();
        ({
            scoped_test_envImpl::create_symlink(
                &field_ptr!(this, env_),
                unimplemented!(),
                unimplemented!(),
                Some(false),
            )
        });
        ({ scoped_test_envImpl::create_dir(&field_ptr!(this, env_), unimplemented!()) });
        ({ scoped_test_envImpl::create_dir(&field_ptr!(this, env_), unimplemented!()) });
        ({ scoped_test_envImpl::create_file(&field_ptr!(this, env_), unimplemented!(), None) });
        ({ scoped_test_envImpl::create_dir(&field_ptr!(this, env_), unimplemented!()) });
        ({ scoped_test_envImpl::create_file(&field_ptr!(this, env_), unimplemented!(), None) });
        ({ scoped_test_envImpl::create_file(&field_ptr!(this, env_), unimplemented!(), None) });
        ({
            scoped_test_envImpl::create_directory_symlink(
                &field_ptr!(this, env_),
                unimplemented!(),
                unimplemented!(),
                Some(false),
            )
        });
        ({ scoped_test_envImpl::create_file(&field_ptr!(this, env_), unimplemented!(), None) });
        ({
            scoped_test_envImpl::create_file(
                &field_ptr!(this, env_),
                unimplemented!(),
                Some(42_u64),
            )
        });
        ({ scoped_test_envImpl::create_file(&field_ptr!(this, env_), unimplemented!(), None) });
        ({
            scoped_test_envImpl::create_file(
                &field_ptr!(this, env_),
                unimplemented!(),
                Some(42_u64),
            )
        });
        ({
            scoped_test_envImpl::create_directory_symlink(
                &field_ptr!(this, env_),
                unimplemented!(),
                unimplemented!(),
                Some(false),
            )
        });
        ({
            scoped_test_envImpl::create_symlink(
                &field_ptr!(this, env_),
                unimplemented!(),
                unimplemented!(),
                Some(false),
            )
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for static_test_env {
    fn default() -> Self {
        { static_test_env::new() }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(24)]
pub struct CWDGuard {
    #[offset(0)]
    #[byte_size(24)]
    pub oldCwd_: (),
}
impl CWDGuard {
    pub fn new() -> Self {
        Self {
            oldCwd_: ({ getcwd_17() }),
        }
    }
}
impl Default for CWDGuard {
    fn default() -> Self {
        { CWDGuard::new() }
    }
}
pub fn GetTestEC_26(Idx: Option<()>) -> () {
    let Idx: Value<()> = Rc::new(RefCell::new(Idx.unwrap_or_else(|| 0_())));
    let GetErrc: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let Idx: Ptr<()> = Idx.as_pointer();
        },
        || -> () {
            switch!(match (Idx.read()) {
                __v if __v == 0_() => {
                    return unimplemented!();
                }
                __v if __v == 1_() => {
                    return unimplemented!();
                }
                __v if __v == 2_() => {
                    return unimplemented!();
                }
                __v if __v == 3_() => {
                    return unimplemented!();
                }
                _ => {
                    unimplemented!();
                    unimplemented!();
                }
            });
            panic!("ub: non-void function does not return a value")
        }
    )));
    return unimplemented!();
}
pub fn ErrorIsImp_27(ec: Ptr<()>, errors: Ptr<()>) -> () {
    let cond: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: for errc in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut errc: () = (unimplemented!().read());
        if (unimplemented!() == (errc as ())) {
            return true;
        }
    }
    return false;
}
pub fn NormalizeExpectedPerms_28(P: ()) -> () {
    let P: Value<()> = Rc::new(RefCell::new(P));
    return (*P.borrow());
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(96)]
pub struct ExceptionChecker {
    #[offset(0)]
    #[byte_size(4)]
    pub expected_err: (),
    #[offset(8)]
    #[byte_size(24)]
    pub expected_path1: (),
    #[offset(32)]
    #[byte_size(24)]
    pub expected_path2: (),
    #[offset(56)]
    pub num_paths: (),
    #[offset(64)]
    #[byte_size(8)]
    pub func_name: Ptr<()>,
    #[offset(72)]
    #[byte_size(24)]
    pub opt_message: (),
}
impl ExceptionChecker {
    pub fn new_1(mut first_err: (), mut fun_name: Ptr<()>, opt_msg: Option<()>) -> Self {
        let mut opt_msg: () = opt_msg.unwrap_or_else(|| unimplemented!());
        Self {
            expected_err: first_err,
            expected_path1: unimplemented!(),
            expected_path2: unimplemented!(),
            num_paths: 0_(),
            func_name: (fun_name).clone(),
            opt_message: unimplemented!(),
        }
    }
    pub fn new_2(p: (), mut first_err: (), mut fun_name: Ptr<()>, opt_msg: Option<()>) -> Self {
        let p: Value<()> = Rc::new(RefCell::new(p));
        let mut opt_msg: () = opt_msg.unwrap_or_else(|| unimplemented!());
        Self {
            expected_err: first_err,
            expected_path1: unimplemented!(),
            expected_path2: unimplemented!(),
            num_paths: 1_(),
            func_name: (fun_name).clone(),
            opt_message: unimplemented!(),
        }
    }
    pub fn new_3(
        p1: (),
        p2: (),
        mut first_err: (),
        mut fun_name: Ptr<()>,
        opt_msg: Option<()>,
    ) -> Self {
        let p1: Value<()> = Rc::new(RefCell::new(p1));
        let p2: Value<()> = Rc::new(RefCell::new(p2));
        let mut opt_msg: () = opt_msg.unwrap_or_else(|| unimplemented!());
        Self {
            expected_err: first_err,
            expected_path1: unimplemented!(),
            expected_path2: unimplemented!(),
            num_paths: 2_(),
            func_name: (fun_name).clone(),
            opt_message: unimplemented!(),
        }
    }
}
impl Default for ExceptionChecker {
    fn default() -> Self {
        {
            ExceptionChecker {
                expected_err: unimplemented!(),
                expected_path1: unimplemented!(),
                expected_path2: unimplemented!(),
                num_paths: unimplemented!(),
                func_name: Ptr::<()>::null(),
                opt_message: unimplemented!(),
            }
        }
    }
}
pub fn GetWindowsInaccessibleDir_29() -> () {
    let dir: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ec: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let root: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let __tmp_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let __range1: Ptr<()> = __tmp_0.as_pointer();
    'loop_: for ent in IterRange::new(unimplemented!(), unimplemented!()) {
        let ent: Ptr<()> = unimplemented!();
        if !(unimplemented!()) {
            continue 'loop_;
        }
        if (!(unimplemented!())) || (!(unimplemented!())) {
            eprintln!(
                "The expected inaccessible directory \"{}\" was found but doesn\'t behave as expected, skipping tests regarding it",
                unimplemented!()
            );
            return unimplemented!();
        }
        &(unimplemented!());
        if !(unimplemented!()) {
            eprintln!(
                "The expected inaccessible directory \"{}\" was found but seems to be accessible, skipping tests regarding it",
                unimplemented!()
            );
            return unimplemented!();
        }
        return unimplemented!();
    }
    eprintln!(
        "No inaccessible directory \"{}\" found, skipping tests regarding it",
        unimplemented!()
    );
    return unimplemented!();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    let env: Value<scoped_test_env> = Rc::new(RefCell::new(scoped_test_env::new()));
    let _dtor_env = ScopedDestructor::new(&env, |__p| __p.destructor());
    let parent_dir: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let child_dir: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut BadRet: u64 = (-1_() as u64);
    let ec: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait CWDGuardImpl {
    fn destructor(&self);
}
impl CWDGuardImpl for Ptr<CWDGuard> {
    fn destructor(&self) {
        let mut ret: () = ({ chdir_25(unimplemented!()) });
        unimplemented!();
    }
}
pub trait ExceptionCheckerImpl {
    fn operator_call(&self, Err_: Ptr<()>);
    fn check_libcxx_string(&self, Err_: Ptr<()>);
}
impl ExceptionCheckerImpl for Ptr<ExceptionChecker> {
    fn operator_call(&self, Err_: Ptr<()>) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        ({ ExceptionCheckerImpl::check_libcxx_string(self, (Err_).clone()) });
    }
    fn check_libcxx_string(&self, Err_: Ptr<()>) {
        let message: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let additional_msg: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        if !(unimplemented!()) {
            unimplemented!();
        }
        let transform_path: Value<FnPtr<fn(Ptr<()>) -> ()>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                |p: Ptr<()>| -> () {
                    {
                        return unimplemented!();
                    }
                },
            )));
        let format: Value<()> = Rc::new(RefCell::new(
            ({
                lambda! ( { let this_ : Value<Ptr< ExceptionChecker  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let additional_msg : Ptr< ()  > = additional_msg .as_pointer()  ;
 let message : Ptr< ()  > = message .as_pointer()  ;
 let transform_path : Ptr< FnPtr<fn(Ptr< ()  > ,) -> ()  >  > = transform_path .as_pointer()  ;
 } , | | -> ()  { 'switch: { match { (*this_.borrow()).clone() .with(|__s| __s . num_paths )  } { __v if __v ==  0_()  =>  { return ( { format_string_8 ( Ptr::<() >::from_string_literal(b"filesystem error: in %s: %s%s" )  , field_ptr!((*this_.borrow()).clone()  , func_name)  , (additional_msg ).clone() , (message ).clone() , ) } )    ;
 }, __v if __v ==  1_()  =>  { return ( { let _args_1: Ptr< Ptr< ()  >  >  = field_ptr!((*this_.borrow()).clone()  , func_name)  ; let _args_4 : Value<Ptr< ()  > > = Rc::new(RefCell::new(unimplemented!()  )); format_string_9 ( Ptr::<() >::from_string_literal(b"filesystem error: in %s: %s%s [%s]" )  , _args_1 , (additional_msg ).clone() , (message ).clone() , _args_4.as_pointer() , ) } )    ;
 }, __v if __v ==  2_()  =>  { return ( { let _args_1: Ptr< Ptr< ()  >  >  = field_ptr!((*this_.borrow()).clone()  , func_name)  ; let _args_4 : Value<Ptr< ()  > > = Rc::new(RefCell::new(unimplemented!()  )); let _args_5 : Value<Ptr< ()  > > = Rc::new(RefCell::new(unimplemented!()  )); format_string_10 ( Ptr::<() >::from_string_literal(b"filesystem error: in %s: %s%s [%s] [%s]" )  , _args_1 , (additional_msg ).clone() , (message ).clone() , _args_4.as_pointer() , _args_5.as_pointer() , ) } )    ;
 }, _ =>  { ( { test_fail_2 ( Ptr::<() >::from_string_literal(b"/app/cpp2rust/tests/libcxx/support/filesystem_test_helper.h" )  , 618_()  , Ptr::<() >::from_string_literal(b"unexpected case" ) , ) } )  ;
 return unimplemented!()   ;
 }, } } ;
 panic!("ub: non-void function does not return a value") } )  .call ( )
            }),
        ));
        unimplemented!();
        if !(unimplemented!()) {
            eprintln!("filesystem_error::what() does not match expected output:");
            eprintln!("  expected: \"{}\"", unimplemented!());
            eprintln!("  actual:   \"{}\"\n", unimplemented!());
        }
    }
}
pub trait GuardVAList_4Impl {
    fn clear(&self);
    fn destructor(&self);
}
impl GuardVAList_4Impl for Ptr<GuardVAList_4> {
    fn clear(&self) {
        if (*self).with(|__s| __s.active) {}
        field!((*self), active).write(false);
    }
    fn destructor(&self) {
        if (*self).with(|__s| __s.active) {}
    }
}
pub trait MultiStringTypeImpl {
    fn as_ptr_2(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_3(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_4(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_5(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_6(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn length_7(&self, _a0: Ptr<()>) -> ();
    fn length_8(&self, _a0: Ptr<()>) -> ();
    fn length_9(&self, _a0: Ptr<()>) -> ();
    fn length_10(&self, _a0: Ptr<()>) -> ();
    fn length_11(&self, _a0: Ptr<()>) -> ();
    fn to_pconst_12(&self) -> Ptr<()>;
    fn to_pconst_13(&self) -> Ptr<()>;
    fn to_pconst_14(&self) -> Ptr<()>;
    fn to_pconst_15(&self) -> Ptr<()>;
    fn to_pconst_16(&self) -> Ptr<()>;
}
impl MultiStringTypeImpl for Ptr<MultiStringType> {
    fn as_ptr_2(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn as_ptr_3(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn as_ptr_4(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn as_ptr_5(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn as_ptr_6(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
    }
    fn length_7(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.sn_);
    }
    fn length_8(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.wn_);
    }
    fn length_9(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u8n_);
    }
    fn length_10(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u16n_);
    }
    fn length_11(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u32n_);
    }
    fn to_pconst_12(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn to_pconst_13(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn to_pconst_14(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn to_pconst_15(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn to_pconst_16(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
    }
}
pub trait scoped_test_envImpl {
    fn destructor(&self);
    fn make_env_path(&self, p: ()) -> ();
    fn sanitize_path(&self, raw_: ()) -> ();
    fn create_file(&self, filename_path: (), size: Option<u64>) -> ();
    fn create_dir(&self, filename_path: ()) -> ();
    fn create_file_dir_symlink(
        &self,
        source_path: (),
        to_path: (),
        sanitize_source: Option<()>,
        is_dir: Option<()>,
    ) -> ();
    fn create_symlink(&self, source_path: (), to_path: (), sanitize_source: Option<()>) -> ();
    fn create_directory_symlink(
        &self,
        source_path: (),
        to_path: (),
        sanitize_source: Option<()>,
    ) -> ();
    fn create_hardlink(&self, source_path: (), to_path: ()) -> ();
    fn create_fifo(&self, file: ()) -> ();
    fn create_socket(&self, file: ()) -> ();
}
impl scoped_test_envImpl for Ptr<scoped_test_env> {
    fn destructor(&self) {
        let mut cmd: () = unimplemented!();
        let mut ret: () = ({ system_20(unimplemented!()) });
        unimplemented!();
        unimplemented!();
        ret = ({ system_20(unimplemented!()) });
        unimplemented!();
    }
    fn make_env_path(&self, mut p: ()) -> () {
        return unimplemented!();
    }
    fn sanitize_path(&self, mut raw_: ()) -> () {
        unimplemented!();
        let mut root: () = unimplemented!();
        if (unimplemented!() != 0_()) {
            unimplemented!();
            let tmp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            return unimplemented!();
        }
        return unimplemented!();
    }
    fn create_file(&self, filename_path: (), size: Option<u64>) -> () {
        let filename_path: Value<()> = Rc::new(RefCell::new(filename_path));
        let mut size: u64 = size.unwrap_or_else(|| 0_u64);
        let mut filename: () = ({ scoped_test_envImpl::sanitize_path(self, unimplemented!()) });
        if (size > unimplemented!()) {
            eprintln!("create_file({}, {}) too large", unimplemented!(), size);
            unimplemented!();
        }
        let mut file: *mut ::libc::FILE =
            ({ fopen64_15(unimplemented!(), Ptr::<()>::from_string_literal(b"we")) });
        if (file).is_null() {
            eprintln!(
                "fopen {} failed: {}",
                unimplemented!(),
                ({ strerror_21((unimplemented!().read()),) })
            );
            unimplemented!();
        }
        if (({ ftruncate64_16(libc::fileno(file), (size as ::libc::off_t)) }) == -1_()) {
            eprintln!(
                "ftruncate {} {} failed: {}",
                unimplemented!(),
                size,
                ({ strerror_21((unimplemented!().read()),) })
            );
            libc::fclose(file);
            unimplemented!();
        }
        libc::fclose(file);
        return unimplemented!();
    }
    fn create_dir(&self, filename_path: ()) -> () {
        let filename_path: Value<()> = Rc::new(RefCell::new(filename_path));
        let mut filename: () = unimplemented!();
        unimplemented!();
        let mut ret: () = unimplemented!();
        unimplemented!();
        return unimplemented!();
    }
    fn create_file_dir_symlink(
        &self,
        source_path: (),
        to_path: (),
        sanitize_source: Option<()>,
        is_dir: Option<()>,
    ) -> () {
        let source_path: Value<()> = Rc::new(RefCell::new(source_path));
        let to_path: Value<()> = Rc::new(RefCell::new(to_path));
        let mut sanitize_source: () = sanitize_source.unwrap_or_else(|| true);
        let mut is_dir: () = is_dir.unwrap_or_else(|| false);
        let mut source: () = unimplemented!();
        let mut to: () = unimplemented!();
        if sanitize_source {
            unimplemented!();
        }
        unimplemented!();
        let mut ret: () = ({
            let _oldname: Ptr<()> = unimplemented!();
            let _newname: Ptr<()> = unimplemented!();
            symlink_11(_oldname, _newname, is_dir)
        });
        unimplemented!();
        return unimplemented!();
    }
    fn create_symlink(&self, source_path: (), to_path: (), sanitize_source: Option<()>) -> () {
        let source_path: Value<()> = Rc::new(RefCell::new(source_path));
        let to_path: Value<()> = Rc::new(RefCell::new(to_path));
        let mut sanitize_source: () = sanitize_source.unwrap_or_else(|| true);
        return ({
            scoped_test_envImpl::create_file_dir_symlink(
                self,
                unimplemented!(),
                unimplemented!(),
                Some(sanitize_source),
                Some(false),
            )
        });
    }
    fn create_directory_symlink(
        &self,
        source_path: (),
        to_path: (),
        sanitize_source: Option<()>,
    ) -> () {
        let source_path: Value<()> = Rc::new(RefCell::new(source_path));
        let to_path: Value<()> = Rc::new(RefCell::new(to_path));
        let mut sanitize_source: () = sanitize_source.unwrap_or_else(|| true);
        return ({
            scoped_test_envImpl::create_file_dir_symlink(
                self,
                unimplemented!(),
                unimplemented!(),
                Some(sanitize_source),
                Some(true),
            )
        });
    }
    fn create_hardlink(&self, source_path: (), to_path: ()) -> () {
        let source_path: Value<()> = Rc::new(RefCell::new(source_path));
        let to_path: Value<()> = Rc::new(RefCell::new(to_path));
        let mut source: () = unimplemented!();
        let mut to: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut ret: () = ({
            let ___from: Ptr<()> = unimplemented!();
            let ___to: Ptr<()> = unimplemented!();
            link_22(___from, ___to)
        });
        unimplemented!();
        return unimplemented!();
    }
    fn create_fifo(&self, mut file: ()) -> () {
        unimplemented!();
        let mut ret: () = ({ mkfifo_23(unimplemented!(), 438_()) });
        unimplemented!();
        return unimplemented!();
    }
    fn create_socket(&self, mut file: ()) -> () {
        unimplemented!();
        let address: Value<libc::sockaddr_un> =
            Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        (*address.borrow_mut()).sun_family = (1 as libc::sa_family_t);
        if (unimplemented!() > ::std::mem::size_of::<[(); 108]>()) {
            unimplemented!();
        }
        unimplemented!();
        ({
            let ___dest: Ptr<()> = (array_field_ptr!(address.as_pointer(), sun_path) as Ptr<()>);
            let ___n: () = ::std::mem::size_of::<[(); 108]>();
            strncpy_24(___dest, unimplemented!(), ___n)
        });
        let mut fd: () = libc::socket(1_(), (libc::SOCK_STREAM as ()), 0_());
        unimplemented!();
        return unimplemented!();
    }
}
pub trait static_test_envImpl {
    fn makePath(&self, p: Ptr<()>) -> ();
    fn destructor(&self);
}
impl static_test_envImpl for Ptr<static_test_env> {
    fn makePath(&self, p: Ptr<()>) -> () {
        let env_path: Ptr<()> = field_ptr!((*self), Root);
        return unimplemented!();
    }
    fn destructor(&self) {
        field_ptr!(self, env_).destructor();
    }
}
pub fn __cpp2rust_init_globals() {}
