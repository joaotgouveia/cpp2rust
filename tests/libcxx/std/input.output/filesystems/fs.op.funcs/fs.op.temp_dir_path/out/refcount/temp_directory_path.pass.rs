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
    unimplemented!();
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
    let mut ret: () = unimplemented!();
    ({ GuardVAList_4Impl::clear(&args_copy_guard.as_pointer()) });
    if (ret == 0_()) {
        return unimplemented!();
    }
    if (ret < size) {
        return unimplemented!();
    }
    size = (ret).wrapping_add(1_());
    let mut buff_ptr: () = unimplemented!();
    ret = unimplemented!();
    return unimplemented!();
}
pub fn unwrap_5(s: Ptr<()>) -> Ptr<()> {
    return unimplemented!();
}
pub fn unwrap_6(a: Ptr<Ptr<()>>) -> Ptr<Ptr<()>> {
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
pub fn format_string_7(
    mut fmt: Ptr<()>,
    args_1: Ptr<Ptr<()>>,
    args_2: Ptr<()>,
    args_3: Ptr<()>,
) -> () {
    return ({
        format_string_imp_3(
            fmt,
            Box::new([
                (({ unwrap_6((args_1).clone()) }).read()).into(),
                ({ unwrap_5((args_2).clone()) }).into(),
                ({ unwrap_5((args_3).clone()) }).into(),
            ]),
        )
    });
}
pub fn format_string_8(
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
                (({ unwrap_6((args_1).clone()) }).read()).into(),
                ({ unwrap_5((args_2).clone()) }).into(),
                ({ unwrap_5((args_3).clone()) }).into(),
                (({ unwrap_6((args_4).clone()) }).read()).into(),
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
    args_5: Ptr<Ptr<()>>,
) -> () {
    return ({
        format_string_imp_3(
            fmt,
            Box::new([
                (({ unwrap_6((args_1).clone()) }).read()).into(),
                ({ unwrap_5((args_2).clone()) }).into(),
                ({ unwrap_5((args_3).clone()) }).into(),
                (({ unwrap_6((args_4).clone()) }).read()).into(),
                (({ unwrap_6((args_5).clone()) }).read()).into(),
            ]),
        )
    });
}
pub fn symlink_10(mut oldname: Ptr<()>, mut newname: Ptr<()>, mut is_dir: ()) -> () {
    &(is_dir);
    return unimplemented!();
}
pub fn space_11(mut path: (), capacity: Ptr<()>, free: Ptr<()>, avail: Ptr<()>) -> () {
    let expect: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    if (unimplemented!() == -1_()) {
        return false;
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let do_mult: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let expect: Ptr<()> = expect.as_pointer();
        },
        |mut val: ()| -> () {
            let mut fsize: () = expect.with(|__s| __s.f_frsize);
            let new_val: Value<()> = Rc::new(RefCell::new((val).wrapping_mul(fsize)));
            unimplemented!();
            return (*new_val.borrow());
        }
    )));
    let __rhs = ({ (*do_mult.borrow()).call({ (*expect.borrow()).f_blocks }) });
    capacity.write(__rhs);
    let __rhs = ({ (*do_mult.borrow()).call({ (*expect.borrow()).f_bfree }) });
    free.write(__rhs);
    let __rhs = ({ (*do_mult.borrow()).call({ (*expect.borrow()).f_bavail }) });
    avail.write(__rhs);
    return true;
}
pub fn fopen64_12(mut pathname: Ptr<()>, mut mode: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn ftruncate64_13(mut fd: (), mut length: ()) -> () {
    return unimplemented!();
}
pub fn getcwd_14() -> () {
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..4096).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    let mut ret: Ptr<()> = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn exists_15(path: Ptr<()>) -> () {
    let tmp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    return (unimplemented!() == 0_());
}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(40)]
pub struct scoped_test_env {
    #[offset(0)]
    #[byte_size(40)]
    pub test_root: (),
}
impl scoped_test_env {
    pub fn new() -> Self {
        let __this: scoped_test_env = Self {
            test_root: ({ scoped_test_env::available_cwd_path() }),
        };
        let mut cmd: () = unimplemented!();
        let mut ret: () = unimplemented!();
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
            exists_15(_path.as_pointer())
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
#[byte_size(664)]
pub struct static_test_env {
    #[offset(0)]
    #[byte_size(40)]
    env_: scoped_test_env,
    #[offset(40)]
    #[byte_size(40)]
    pub Root: (),
    #[offset(80)]
    #[byte_size(24)]
    pub TestFileList: (),
    #[offset(104)]
    #[byte_size(24)]
    pub TestDirList: (),
    #[offset(128)]
    #[byte_size(40)]
    pub File: (),
    #[offset(168)]
    #[byte_size(40)]
    pub Dir: (),
    #[offset(208)]
    #[byte_size(40)]
    pub Dir2: (),
    #[offset(248)]
    #[byte_size(40)]
    pub Dir3: (),
    #[offset(288)]
    #[byte_size(40)]
    pub SymlinkToFile: (),
    #[offset(328)]
    #[byte_size(40)]
    pub SymlinkToDir: (),
    #[offset(368)]
    #[byte_size(40)]
    pub BadSymlink: (),
    #[offset(408)]
    #[byte_size(40)]
    pub DNE: (),
    #[offset(448)]
    #[byte_size(40)]
    pub EmptyFile: (),
    #[offset(488)]
    #[byte_size(40)]
    pub NonEmptyFile: (),
    #[offset(528)]
    #[byte_size(40)]
    pub CharFile: (),
    #[offset(568)]
    #[byte_size(24)]
    pub DirIterationList: (),
    #[offset(592)]
    #[byte_size(24)]
    pub DirIterationListDepth1: (),
    #[offset(616)]
    #[byte_size(24)]
    pub RecDirIterationList: (),
    #[offset(640)]
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
            scoped_test_envImpl::create_file(&field_ptr!(this, env_), unimplemented!(), Some(42_()))
        });
        ({ scoped_test_envImpl::create_file(&field_ptr!(this, env_), unimplemented!(), None) });
        ({
            scoped_test_envImpl::create_file(&field_ptr!(this, env_), unimplemented!(), Some(42_()))
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
#[byte_size(32)]
pub struct CWDGuard {
    #[offset(0)]
    #[byte_size(32)]
    pub oldCwd_: (),
}
impl CWDGuard {
    pub fn new() -> Self {
        Self {
            oldCwd_: ({ getcwd_14() }),
        }
    }
}
impl Default for CWDGuard {
    fn default() -> Self {
        { CWDGuard::new() }
    }
}
pub fn GetTestEC_16(Idx: Option<()>) -> () {
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
pub fn ErrorIsImp_17(ec: Ptr<()>, errors: Ptr<()>) -> () {
    let cond: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: for errc in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut errc: () = (unimplemented!().read());
        if (unimplemented!() == (errc as ())) {
            return true;
        }
    }
    return false;
}
pub fn ErrorIs_18(ec: Ptr<()>, mut First: ()) -> () {
    let errors: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    return ({
        let _ec: Ptr<()> = (ec).clone();
        let _errors: Ptr<()> = errors.as_pointer();
        ErrorIsImp_17(_ec, _errors)
    });
}
pub fn NormalizeExpectedPerms_19(P: ()) -> () {
    let P: Value<()> = Rc::new(RefCell::new(P));
    return (*P.borrow());
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(136)]
pub struct ExceptionChecker {
    #[offset(0)]
    #[byte_size(4)]
    pub expected_err: (),
    #[offset(8)]
    #[byte_size(40)]
    pub expected_path1: (),
    #[offset(48)]
    #[byte_size(40)]
    pub expected_path2: (),
    #[offset(88)]
    pub num_paths: (),
    #[offset(96)]
    #[byte_size(8)]
    pub func_name: Ptr<()>,
    #[offset(104)]
    #[byte_size(32)]
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
pub fn GetWindowsInaccessibleDir_20() -> () {
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
pub fn PutEnv_21(mut var: (), value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    unimplemented!();
}
pub fn UnsetEnv_22(mut var: ()) {
    unimplemented!();
}
pub fn signature_test_23() {
    let ec: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (&(*ec.borrow_mut()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn basic_tests_24() {
    let env: Value<scoped_test_env> = Rc::new(RefCell::new(scoped_test_env::new()));
    let _dtor_env = ScopedDestructor::new(&env, |__p| __p.destructor());
    let dne: Value<()> = Rc::new(RefCell::new(
        ({ scoped_test_envImpl::make_env_path(&env.as_pointer(), unimplemented!()) }),
    ));
    let file: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let dir_perms: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let inaccessible_dir: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    const _: () = assert!(true, "");;
    let cases: Value<Box<[TestCase_25]>> = Rc::new(RefCell::new(Box::new([
        TestCase_25 {
            name: unimplemented!(),
            p: unimplemented!(),
        },
        TestCase_25 {
            name: unimplemented!(),
            p: unimplemented!(),
        },
        TestCase_25 {
            name: unimplemented!(),
            p: unimplemented!(),
        },
        TestCase_25 {
            name: unimplemented!(),
            p: unimplemented!(),
        },
    ])));
    let ignored_cases: Value<Box<[TestCase_25]>> = Rc::new(RefCell::new(Box::new([TestCase_25 {
        name: unimplemented!(),
        p: unimplemented!(),
    }])));
    'loop_: for TC in IterRange::new(
        (cases.as_pointer() as Ptr<TestCase_25>),
        (cases.as_pointer() as Ptr<TestCase_25>).offset((4_()) as isize),
    ) {
        let TC: Ptr<TestCase_25> = (TC).clone();
        ({
            let _var: () = unimplemented!();
            let _value: () = unimplemented!();
            PutEnv_21(_var, _value)
        });
    }
    'loop_: for TC in IterRange::new(
        (cases.as_pointer() as Ptr<TestCase_25>),
        (cases.as_pointer() as Ptr<TestCase_25>).offset((4_()) as isize),
    ) {
        let TC: Ptr<TestCase_25> = (TC).clone();
        let ec: Value<()> = Rc::new(RefCell::new(({ GetTestEC_16(None) })));
        let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        ({ PutEnv_21(unimplemented!(), unimplemented!()) });
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        ({ PutEnv_21(unimplemented!(), unimplemented!()) });
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        if !(unimplemented!()) {
            ({ PutEnv_21(unimplemented!(), unimplemented!()) });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({
            let _var: () = unimplemented!();
            let _value: () = unimplemented!();
            PutEnv_21(_var, _value)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        ({ UnsetEnv_22(unimplemented!()) });
    }
    let fallback: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let ec: Value<()> = Rc::new(RefCell::new(({ GetTestEC_16(None) })));
        let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    };
    'loop_: for TC in IterRange::new(
        (ignored_cases.as_pointer() as Ptr<TestCase_25>),
        (ignored_cases.as_pointer() as Ptr<TestCase_25>).offset((1_()) as isize),
    ) {
        let TC: Ptr<TestCase_25> = (TC).clone();
        ({
            let _var: () = unimplemented!();
            let _value: () = unimplemented!();
            PutEnv_21(_var, _value)
        });
        let ec: Value<()> = Rc::new(RefCell::new(({ GetTestEC_16(None) })));
        let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        ({ UnsetEnv_22(unimplemented!()) });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(72)]
pub struct TestCase_25 {
    #[offset(0)]
    #[byte_size(32)]
    pub name: (),
    #[offset(32)]
    #[byte_size(40)]
    pub p: (),
}
impl TestCase_25 {
    pub fn move_from(_a0: Ptr<TestCase_25>) -> Self {
        Self {
            name: unimplemented!(),
            p: unimplemented!(),
        }
    }
}
impl Default for TestCase_25 {
    fn default() -> Self {
        {
            TestCase_25 {
                name: unimplemented!(),
                p: unimplemented!(),
            }
        }
    }
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
    ({ signature_test_23() });
    ({ basic_tests_24() });
    return 0_();
}
pub trait CWDGuardImpl {
    fn destructor(&self);
}
impl CWDGuardImpl for Ptr<CWDGuard> {
    fn destructor(&self) {
        let mut ret: () = unimplemented!();
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
        const _: () = assert!(true, "");;
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
 } , | | -> ()  { 'switch: { match { (*this_.borrow()).clone() .with(|__s| __s . num_paths )  } { __v if __v ==  0_()  =>  { return ( { format_string_7 ( Ptr::<() >::from_string_literal(b"filesystem error: in %s: %s%s" )  , field_ptr!((*this_.borrow()).clone()  , func_name)  , (additional_msg ).clone() , (message ).clone() , ) } )    ;
 }, __v if __v ==  1_()  =>  { return ( { let _args_1: Ptr< Ptr< ()  >  >  = field_ptr!((*this_.borrow()).clone()  , func_name)  ; let _args_4 : Value<Ptr< ()  > > = Rc::new(RefCell::new(unimplemented!()  )); format_string_8 ( Ptr::<() >::from_string_literal(b"filesystem error: in %s: %s%s [%s]" )  , _args_1 , (additional_msg ).clone() , (message ).clone() , _args_4.as_pointer() , ) } )    ;
 }, __v if __v ==  2_()  =>  { return ( { let _args_1: Ptr< Ptr< ()  >  >  = field_ptr!((*this_.borrow()).clone()  , func_name)  ; let _args_4 : Value<Ptr< ()  > > = Rc::new(RefCell::new(unimplemented!()  )); let _args_5 : Value<Ptr< ()  > > = Rc::new(RefCell::new(unimplemented!()  )); format_string_9 ( Ptr::<() >::from_string_literal(b"filesystem error: in %s: %s%s [%s] [%s]" )  , _args_1 , (additional_msg ).clone() , (message ).clone() , _args_4.as_pointer() , _args_5.as_pointer() , ) } )    ;
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
pub trait TestCase_25Impl {
    fn move_assign(&self, _a0: Ptr<TestCase_25>) -> Ptr<TestCase_25>;
}
impl TestCase_25Impl for Ptr<TestCase_25> {
    fn move_assign(&self, _a0: Ptr<TestCase_25>) -> Ptr<TestCase_25> {
        unimplemented!();
        unimplemented!();
        return (*self).clone();
    }
}
pub trait scoped_test_envImpl {
    fn destructor(&self);
    fn make_env_path(&self, p: ()) -> ();
    fn sanitize_path(&self, raw_: ()) -> ();
    fn create_file(&self, filename_path: (), size: Option<()>) -> ();
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
        let mut ret: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        ret = unimplemented!();
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
    fn create_file(&self, filename_path: (), size: Option<()>) -> () {
        let filename_path: Value<()> = Rc::new(RefCell::new(filename_path));
        let mut size: () = size.unwrap_or_else(|| 0_());
        let mut filename: () = ({ scoped_test_envImpl::sanitize_path(self, unimplemented!()) });
        if (size > unimplemented!()) {
            eprintln!("create_file({}, {}) too large", unimplemented!(), size);
            unimplemented!();
        }
        let mut file: () =
            ({ fopen64_12(unimplemented!(), Ptr::<()>::from_string_literal(b"we")) });
        if (file).is_null() {
            eprintln!("fopen {} failed: {}", unimplemented!(), unimplemented!());
            unimplemented!();
        }
        if (({ ftruncate64_13(unimplemented!(), size) }) == -1_()) {
            eprintln!(
                "ftruncate {} {} failed: {}",
                unimplemented!(),
                size,
                unimplemented!()
            );
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
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
            symlink_10(_oldname, _newname, is_dir)
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
        let mut ret: () = unimplemented!();
        unimplemented!();
        return unimplemented!();
    }
    fn create_fifo(&self, mut file: ()) -> () {
        unimplemented!();
        let mut ret: () = unimplemented!();
        unimplemented!();
        return unimplemented!();
    }
    fn create_socket(&self, mut file: ()) -> () {
        unimplemented!();
        let address: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*address.borrow_mut()).sun_family = 1_();
        if (unimplemented!() > ::std::mem::size_of::<[(); 108]>()) {
            unimplemented!();
        }
        unimplemented!();
        unimplemented!();
        let mut fd: () = unimplemented!();
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
