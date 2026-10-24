extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_eprintf_0(
    mut fmt: *const (),
    args_1: *const *const (),
    args_2: *const *const (),
    args_3: *const *const (),
    args_4: *const (),
    args_5: *const *const (),
) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args_1),
            (*args_2),
            (*args_3),
            (*args_4),
            (*args_5),
        )
    });
}
pub unsafe fn test_log_1(
    mut condition: *const (),
    mut file: *const (),
    mut line: (),
    mut message: *const (),
) {
    let mut msg: *const () = if !(condition).is_null() {
        c"Assertion failure: ".as_ptr()
    } else {
        c"Unconditional failure:".as_ptr()
    };
    (unsafe {
        test_eprintf_0(
            c"%s%s %s %d\n%s".as_ptr(),
            &msg,
            &condition,
            &file,
            &line,
            &message,
        )
    });
}
pub unsafe fn test_fail_2(mut file: *const (), mut line: (), arg: *const [(); 16]) {
    (unsafe { test_log_1(c"".as_ptr(), file, line, (*arg).as_ptr()) });
    unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MultiStringType {
    pub w_: *const (),
    pub wn_: (),
    pub u8_: *const (),
    pub u8n_: (),
    pub u16_: *const (),
    pub u16n_: (),
    pub u32_: *const (),
    pub u32n_: (),
    pub s_: *const (),
    pub sn_: (),
}
impl MultiStringType {
    pub unsafe fn new(
        mut w: *const (),
        mut wn: (),
        mut u8: *const (),
        mut u8n: (),
        mut u16: *const (),
        mut u16n: (),
        mut u32: *const (),
        mut u32n: (),
        mut s: *const (),
        mut sn: (),
    ) -> Self {
        let mut this = Self {
            w_: w,
            wn_: wn,
            u8_: u8,
            u8n_: u8n,
            u16_: u16,
            u16n_: u16n,
            u32_: u32,
            u32n_: u32n,
            s_: s,
            sn_: sn,
        };
        this
    }
    pub unsafe fn as_ptr_2(&self, mut _a0: *const ()) -> *const () {
        return self.s_;
    }
    pub unsafe fn as_ptr_3(&self, mut _a0: *const ()) -> *const () {
        return self.w_;
    }
    pub unsafe fn as_ptr_4(&self, mut _a0: *const ()) -> *const () {
        return self.u8_;
    }
    pub unsafe fn as_ptr_5(&self, mut _a0: *const ()) -> *const () {
        return self.u16_;
    }
    pub unsafe fn as_ptr_6(&self, mut _a0: *const ()) -> *const () {
        return self.u32_;
    }
    pub unsafe fn length_7(&self, mut _a0: *const ()) -> () {
        return self.sn_;
    }
    pub unsafe fn length_8(&self, mut _a0: *const ()) -> () {
        return self.wn_;
    }
    pub unsafe fn length_9(&self, mut _a0: *const ()) -> () {
        return self.u8n_;
    }
    pub unsafe fn length_10(&self, mut _a0: *const ()) -> () {
        return self.u16n_;
    }
    pub unsafe fn length_11(&self, mut _a0: *const ()) -> () {
        return self.u32n_;
    }
    pub unsafe fn to_pconst_12(&self) -> *const () {
        return self.s_;
    }
    pub unsafe fn to_pconst_13(&self) -> *const () {
        return self.w_;
    }
    pub unsafe fn to_pconst_14(&self) -> *const () {
        return self.u8_;
    }
    pub unsafe fn to_pconst_15(&self) -> *const () {
        return self.u16_;
    }
    pub unsafe fn to_pconst_16(&self) -> *const () {
        return self.u32_;
    }
}
pub unsafe fn format_string_imp_3(mut msg: *const (), __args: Box<[VaArg]>) -> () {
    let mut args: *const VaArg = ::std::ptr::null();
    args = __args.as_ptr();
    let mut args_guard: GuardVAList_4 = GuardVAList_4::new({ &mut args });
    let _dtor_args_guard =
        ScopedDestructorUnsafe::new(&raw mut args_guard, GuardVAList_4::destructor);
    let mut args_cp: *const VaArg = ::std::ptr::null();
    args_cp = args.clone();
    let mut args_copy_guard: GuardVAList_4 = GuardVAList_4::new({ &mut args_cp });
    let _dtor_args_copy_guard =
        ScopedDestructorUnsafe::new(&raw mut args_copy_guard, GuardVAList_4::destructor);
    let mut local_buff: () = unimplemented!();
    let mut size: () = unimplemented!();
    let mut ret: () = (unsafe { vsnprintf_5(unimplemented!(), size, msg, args_cp) });
    (unsafe { GuardVAList_4::clear(&mut args_copy_guard) });
    if ((ret) == (0_())) {
        return unimplemented!();
    }
    if ((ret) < (size)) {
        return unimplemented!();
    }
    size = (ret).wrapping_add(1_());
    let mut buff_ptr: () = unimplemented!();
    ret = (unsafe { vsnprintf_5(unimplemented!(), size, msg, args) });
    return unimplemented!();
}
pub unsafe fn unwrap_6(s: *mut ()) -> *const () {
    return unimplemented!();
}
pub unsafe fn unwrap_7(a: *mut *const ()) -> *const *const () {
    const _: () = assert!(true, "!std::is_class<Arg>::value");;
    return a;
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct GuardVAList_4 {
    pub xtarget: *mut *const VaArg,
    pub active: (),
}
impl GuardVAList_4 {
    pub unsafe fn new(val: *mut *const VaArg) -> Self {
        let mut this = Self {
            xtarget: val,
            active: true,
        };
        this
    }
    pub unsafe fn clear(&mut self) {
        if self.active {}
        self.active = false;
    }
    pub unsafe fn destructor(&mut self) {
        if self.active {}
    }
}
pub unsafe fn format_string_8(
    mut fmt: *const (),
    args_1: *const *const (),
    args_2: *const (),
    args_3: *const (),
) -> () {
    return (unsafe {
        format_string_imp_3(
            fmt,
            Box::new([
                (*(unsafe { unwrap_7((args_1).cast_mut()) })).into(),
                (unsafe { unwrap_6((args_2).cast_mut()) }).into(),
                (unsafe { unwrap_6((args_3).cast_mut()) }).into(),
            ]),
        )
    });
}
pub unsafe fn format_string_9(
    mut fmt: *const (),
    args_1: *const *const (),
    args_2: *const (),
    args_3: *const (),
    args_4: *const *const (),
) -> () {
    return (unsafe {
        format_string_imp_3(
            fmt,
            Box::new([
                (*(unsafe { unwrap_7((args_1).cast_mut()) })).into(),
                (unsafe { unwrap_6((args_2).cast_mut()) }).into(),
                (unsafe { unwrap_6((args_3).cast_mut()) }).into(),
                (*(unsafe { unwrap_7((args_4).cast_mut()) })).into(),
            ]),
        )
    });
}
pub unsafe fn format_string_10(
    mut fmt: *const (),
    args_1: *const *const (),
    args_2: *const (),
    args_3: *const (),
    args_4: *const *const (),
    args_5: *const *const (),
) -> () {
    return (unsafe {
        format_string_imp_3(
            fmt,
            Box::new([
                (*(unsafe { unwrap_7((args_1).cast_mut()) })).into(),
                (unsafe { unwrap_6((args_2).cast_mut()) }).into(),
                (unsafe { unwrap_6((args_3).cast_mut()) }).into(),
                (*(unsafe { unwrap_7((args_4).cast_mut()) })).into(),
                (*(unsafe { unwrap_7((args_5).cast_mut()) })).into(),
            ]),
        )
    });
}
pub unsafe fn symlink_11(mut oldname: *const (), mut newname: *const (), mut is_dir: ()) -> () {
    &(is_dir);
    return (unsafe { symlink_12(oldname, newname) });
}
pub unsafe fn space_13(mut path: (), capacity: *mut u64, free: *mut u64, avail: *mut u64) -> () {
    let mut expect: statvfs = unsafe { std::mem::zeroed::<statvfs>() };
    if ((unsafe { statvfs_14(unimplemented!(), (&mut expect as *mut statvfs)) }) == (-1_())) {
        return false;
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut do_mult: FnPtr<fn(u64) -> u64> = lambda_unsafe!(
        {
            let expect: *mut statvfs = &mut expect;
        },
        |mut val: u64| -> u64 {
            let mut fsize: u64 = ((*expect).f_frsize as u64);
            let mut new_val: u64 = (val).wrapping_mul(fsize);
            unimplemented!();
            return new_val;
        }
    );
    (*capacity) = (unsafe { do_mult.call((expect.f_blocks as u64)) });
    (*free) = (unsafe { do_mult.call((expect.f_bfree as u64)) });
    (*avail) = (unsafe { do_mult.call((expect.f_bavail as u64)) });
    return true;
}
pub unsafe fn fopen64_15(mut pathname: *const (), mut mode: *const ()) -> *mut ::libc::FILE {
    return libc::fopen(pathname, mode);
}
pub unsafe fn ftruncate64_16(mut fd: (), mut length: ::libc::off_t) -> () {
    return unimplemented!();
}
pub unsafe fn getcwd_17() -> () {
    let mut buf: [(); 4096] = [unimplemented!(); 4096];
    let mut ret: *mut () = (unsafe {
        let ___buf: *mut () = buf.as_mut_ptr();
        let ___size: () = ::std::mem::size_of::<[(); 4096]>();
        getcwd_18(___buf, ___size)
    });
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn exists_19(path: *const ()) -> () {
    let mut tmp: () = unimplemented!();
    return ((unimplemented!()) == (0_()));
}
#[repr(C)]
#[derive(FnPtrArg, DestructorUnsafe)]
pub struct scoped_test_env {
    pub test_root: (),
}
impl scoped_test_env {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            test_root: (unsafe { scoped_test_env::available_cwd_path() }),
        };
        let mut cmd: () = unimplemented!();
        let mut ret: () = (unsafe { system_20(unimplemented!()) });
        unimplemented!();
        unimplemented!();
        this
    }
    pub unsafe fn destructor(&mut self) {
        let mut cmd: () = unimplemented!();
        let mut ret: () = (unsafe { system_20(unimplemented!()) });
        unimplemented!();
        unimplemented!();
        ret = (unsafe { system_20(unimplemented!()) });
        unimplemented!();
    }
    pub unsafe fn make_env_path(&mut self, mut p: ()) -> () {
        return unimplemented!();
    }
    pub unsafe fn sanitize_path(&mut self, mut raw_: ()) -> () {
        unimplemented!();
        let mut root: () = unimplemented!();
        if ((unimplemented!()) != (0_())) {
            unimplemented!();
            let mut tmp: () = unimplemented!();
            unimplemented!();
            return unimplemented!();
        }
        return unimplemented!();
    }
    pub unsafe fn create_file(&mut self, mut filename_path: (), mut size: Option<u64>) -> () {
        let mut size: u64 = size.unwrap_or_else(|| unsafe { 0_u64 });
        let mut filename: () = (unsafe { scoped_test_env::sanitize_path(self, unimplemented!()) });
        if ((size) > (unimplemented!())) {
            (unsafe {
                libc::fprintf(
                    libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
                    c"create_file(%s, %ju) too large\n".as_ptr() as *const libc::c_char,
                    (unimplemented!()),
                    (size),
                )
            });
            unimplemented!();
        }
        let mut file: *mut ::libc::FILE = (unsafe { fopen64_15(unimplemented!(), c"we".as_ptr()) });
        if (file).is_null() {
            (unsafe {
                libc::fprintf(
                    libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
                    c"fopen %s failed: %s\n".as_ptr() as *const libc::c_char,
                    (unimplemented!()),
                    (unsafe { strerror_21((*unimplemented!())) }),
                )
            });
            unimplemented!();
        }
        if ((unsafe { ftruncate64_16(libc::fileno(file), (size as ::libc::off_t)) }) == (-1_())) {
            (unsafe {
                libc::fprintf(
                    libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
                    c"ftruncate %s %ju failed: %s\n".as_ptr() as *const libc::c_char,
                    (unimplemented!()),
                    (size),
                    (unsafe { strerror_21((*unimplemented!())) }),
                )
            });
            libc::fclose(file);
            unimplemented!();
        }
        libc::fclose(file);
        return unimplemented!();
    }
    pub unsafe fn create_dir(&mut self, mut filename_path: ()) -> () {
        let mut filename: () = unimplemented!();
        unimplemented!();
        let mut ret: () = unimplemented!();
        unimplemented!();
        return unimplemented!();
    }
    pub unsafe fn create_file_dir_symlink(
        &mut self,
        mut source_path: (),
        mut to_path: (),
        mut sanitize_source: Option<()>,
        mut is_dir: Option<()>,
    ) -> () {
        let mut sanitize_source: () = sanitize_source.unwrap_or_else(|| unsafe { true });
        let mut is_dir: () = is_dir.unwrap_or_else(|| unsafe { false });
        let mut source: () = unimplemented!();
        let mut to: () = unimplemented!();
        if sanitize_source {
            unimplemented!();
        }
        unimplemented!();
        let mut ret: () = (unsafe {
            let _oldname: *const () = unimplemented!();
            let _newname: *const () = unimplemented!();
            symlink_11(_oldname, _newname, is_dir)
        });
        unimplemented!();
        return unimplemented!();
    }
    pub unsafe fn create_symlink(
        &mut self,
        mut source_path: (),
        mut to_path: (),
        mut sanitize_source: Option<()>,
    ) -> () {
        let mut sanitize_source: () = sanitize_source.unwrap_or_else(|| unsafe { true });
        return (unsafe {
            scoped_test_env::create_file_dir_symlink(
                self,
                unimplemented!(),
                unimplemented!(),
                Some(sanitize_source),
                Some(false),
            )
        });
    }
    pub unsafe fn create_directory_symlink(
        &mut self,
        mut source_path: (),
        mut to_path: (),
        mut sanitize_source: Option<()>,
    ) -> () {
        let mut sanitize_source: () = sanitize_source.unwrap_or_else(|| unsafe { true });
        return (unsafe {
            scoped_test_env::create_file_dir_symlink(
                self,
                unimplemented!(),
                unimplemented!(),
                Some(sanitize_source),
                Some(true),
            )
        });
    }
    pub unsafe fn create_hardlink(&mut self, mut source_path: (), mut to_path: ()) -> () {
        let mut source: () = unimplemented!();
        let mut to: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut ret: () = (unsafe {
            let ___from: *const () = unimplemented!();
            let ___to: *const () = unimplemented!();
            link_22(___from, ___to)
        });
        unimplemented!();
        return unimplemented!();
    }
    pub unsafe fn create_fifo(&mut self, mut file: ()) -> () {
        unimplemented!();
        let mut ret: () = (unsafe { mkfifo_23(unimplemented!(), 438_()) });
        unimplemented!();
        return unimplemented!();
    }
    pub unsafe fn create_socket(&mut self, mut file: ()) -> () {
        unimplemented!();
        let mut address: libc::sockaddr_un = unsafe { std::mem::zeroed() };
        address.sun_family = (1 as libc::sa_family_t);
        if ((unimplemented!()) > (::std::mem::size_of::<[(); 108]>())) {
            unimplemented!();
        }
        unimplemented!();
        (unsafe {
            let ___dest: *mut () = address.sun_path.as_mut_ptr();
            let ___n: () = ::std::mem::size_of::<[(); 108]>();
            strncpy_24(___dest, unimplemented!(), ___n)
        });
        let mut fd: () = libc::socket(1_(), (libc::SOCK_STREAM as ()), 0_());
        unimplemented!();
        return unimplemented!();
    }
    unsafe fn available_cwd_path() -> () {
        let cwd: () = unimplemented!();
        let tmp: () = unimplemented!();
        let mut base: () = unimplemented!();
        let mut i: () = unimplemented!();
        let mut p: () = unimplemented!();
        'loop_: while (unsafe {
            let mut _path: () = unimplemented!();
            exists_19(&mut _path)
        }) {
            unimplemented!();
        }
        return unimplemented!();
    }
}
impl Default for scoped_test_env {
    fn default() -> Self {
        unsafe { scoped_test_env::new() }
    }
}
#[repr(C)]
#[derive(FnPtrArg, DestructorUnsafe)]
pub struct static_test_env {
    env_: scoped_test_env,
    pub Root: (),
    pub TestFileList: (),
    pub TestDirList: (),
    pub File: (),
    pub Dir: (),
    pub Dir2: (),
    pub Dir3: (),
    pub SymlinkToFile: (),
    pub SymlinkToDir: (),
    pub BadSymlink: (),
    pub DNE: (),
    pub EmptyFile: (),
    pub NonEmptyFile: (),
    pub CharFile: (),
    pub DirIterationList: (),
    pub DirIterationListDepth1: (),
    pub RecDirIterationList: (),
    pub RecDirFollowSymlinksIterationList: (),
}
impl static_test_env {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            env_: scoped_test_env::new(),
            Root: unimplemented!(),
            TestFileList: unimplemented!(),
            TestDirList: unimplemented!(),
            File: unimplemented!(),
            Dir: unimplemented!(),
            Dir2: unimplemented!(),
            Dir3: unimplemented!(),
            SymlinkToFile: (unsafe {
                let mut _p: () = unimplemented!();
                static_test_env::makePath(&mut this, &mut _p)
            }),
            SymlinkToDir: (unsafe {
                let mut _p: () = unimplemented!();
                static_test_env::makePath(&mut this, &mut _p)
            }),
            BadSymlink: (unsafe {
                let mut _p: () = unimplemented!();
                static_test_env::makePath(&mut this, &mut _p)
            }),
            DNE: (unsafe {
                let mut _p: () = unimplemented!();
                static_test_env::makePath(&mut this, &mut _p)
            }),
            EmptyFile: unimplemented!(),
            NonEmptyFile: unimplemented!(),
            CharFile: unimplemented!(),
            DirIterationList: unimplemented!(),
            DirIterationListDepth1: unimplemented!(),
            RecDirIterationList: unimplemented!(),
            RecDirFollowSymlinksIterationList: unimplemented!(),
        };
        (unsafe {
            scoped_test_env::create_symlink(
                &mut this.env_,
                unimplemented!(),
                unimplemented!(),
                Some(false),
            )
        });
        (unsafe { scoped_test_env::create_dir(&mut this.env_, unimplemented!()) });
        (unsafe { scoped_test_env::create_dir(&mut this.env_, unimplemented!()) });
        (unsafe { scoped_test_env::create_file(&mut this.env_, unimplemented!(), None) });
        (unsafe { scoped_test_env::create_dir(&mut this.env_, unimplemented!()) });
        (unsafe { scoped_test_env::create_file(&mut this.env_, unimplemented!(), None) });
        (unsafe { scoped_test_env::create_file(&mut this.env_, unimplemented!(), None) });
        (unsafe {
            scoped_test_env::create_directory_symlink(
                &mut this.env_,
                unimplemented!(),
                unimplemented!(),
                Some(false),
            )
        });
        (unsafe { scoped_test_env::create_file(&mut this.env_, unimplemented!(), None) });
        (unsafe { scoped_test_env::create_file(&mut this.env_, unimplemented!(), Some(42_u64)) });
        (unsafe { scoped_test_env::create_file(&mut this.env_, unimplemented!(), None) });
        (unsafe { scoped_test_env::create_file(&mut this.env_, unimplemented!(), Some(42_u64)) });
        (unsafe {
            scoped_test_env::create_directory_symlink(
                &mut this.env_,
                unimplemented!(),
                unimplemented!(),
                Some(false),
            )
        });
        (unsafe {
            scoped_test_env::create_symlink(
                &mut this.env_,
                unimplemented!(),
                unimplemented!(),
                Some(false),
            )
        });
        this
    }
    pub unsafe fn makePath(&self, p: *const ()) -> () {
        let env_path: *const () = &self.Root;
        return unimplemented!();
    }
}
impl static_test_env {
    pub unsafe fn destructor(&mut self) {
        scoped_test_env::destructor(&mut self.env_);
    }
}
impl Default for static_test_env {
    fn default() -> Self {
        unsafe { static_test_env::new() }
    }
}
#[repr(C)]
#[derive(FnPtrArg, DestructorUnsafe)]
pub struct CWDGuard {
    pub oldCwd_: (),
}
impl CWDGuard {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            oldCwd_: (unsafe { getcwd_17() }),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        let mut ret: () = (unsafe { chdir_25(unimplemented!()) });
        unimplemented!();
    }
}
impl Default for CWDGuard {
    fn default() -> Self {
        unsafe { CWDGuard::new() }
    }
}
pub unsafe fn GetTestEC_26(mut Idx: Option<()>) -> () {
    let mut Idx: () = Idx.unwrap_or_else(|| unsafe { 0_() });
    let mut GetErrc: FnPtr<fn() -> ()> = lambda_unsafe!(
        {
            let Idx: *mut () = &mut Idx;
        },
        || -> () {
            switch!(match (*Idx) {
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
    );
    return unimplemented!();
}
pub unsafe fn ErrorIsImp_27(ec: *const (), errors: *const ()) -> () {
    let mut cond: () = unimplemented!();
    'loop_: for errc in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut errc: () = (*unimplemented!());
        if ((unimplemented!()) == (errc as ())) {
            return true;
        }
    }
    return false;
}
pub unsafe fn NormalizeExpectedPerms_28(mut P: ()) -> () {
    return P;
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct ExceptionChecker {
    pub expected_err: (),
    pub expected_path1: (),
    pub expected_path2: (),
    pub num_paths: (),
    pub func_name: *const (),
    pub opt_message: (),
}
impl ExceptionChecker {
    pub unsafe fn new_1(
        mut first_err: (),
        mut fun_name: *const (),
        mut opt_msg: Option<()>,
    ) -> Self {
        let mut opt_msg: () = opt_msg.unwrap_or_else(|| unsafe { unimplemented!() });
        let mut this = Self {
            expected_err: first_err,
            expected_path1: unimplemented!(),
            expected_path2: unimplemented!(),
            num_paths: 0_(),
            func_name: fun_name,
            opt_message: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_2(
        mut p: (),
        mut first_err: (),
        mut fun_name: *const (),
        mut opt_msg: Option<()>,
    ) -> Self {
        let mut opt_msg: () = opt_msg.unwrap_or_else(|| unsafe { unimplemented!() });
        let mut this = Self {
            expected_err: first_err,
            expected_path1: unimplemented!(),
            expected_path2: unimplemented!(),
            num_paths: 1_(),
            func_name: fun_name,
            opt_message: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_3(
        mut p1: (),
        mut p2: (),
        mut first_err: (),
        mut fun_name: *const (),
        mut opt_msg: Option<()>,
    ) -> Self {
        let mut opt_msg: () = opt_msg.unwrap_or_else(|| unsafe { unimplemented!() });
        let mut this = Self {
            expected_err: first_err,
            expected_path1: unimplemented!(),
            expected_path2: unimplemented!(),
            num_paths: 2_(),
            func_name: fun_name,
            opt_message: unimplemented!(),
        };
        this
    }
    pub unsafe fn operator_call(&mut self, Err_: *const ()) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { ExceptionChecker::check_libcxx_string(self, Err_) });
    }
    pub unsafe fn check_libcxx_string(&mut self, Err_: *const ()) {
        let mut message: () = unimplemented!();
        let mut additional_msg: () = unimplemented!();
        if !(unimplemented!()) {
            unimplemented!();
        }
        let mut transform_path: FnPtr<fn(*const ()) -> ()> =
            FnPtr::<fn(*const ()) -> ()>::new(|p: *const ()| -> () {
                unsafe {
                    return unimplemented!();
                }
            });
        let mut format: () = (unsafe {
            lambda_unsafe! ( { let this_ : *mut ExceptionChecker = ( self as *mut ExceptionChecker  ) ;
 let additional_msg : *mut () = & mut additional_msg ;
 let message : *mut () = & mut message ;
 let transform_path : *mut FnPtr<fn(*const () ,) -> () > = & mut transform_path ;
 } , | | -> () { 'switch: { match { ( * this_  ) . num_paths   } { __v if __v ==  0_()  =>  { return ( unsafe { format_string_8 ( c"filesystem error: in %s: %s%s" .as_ptr() , & ( * this_  ) . func_name  , additional_msg , message , ) } )   ;
 }, __v if __v ==  1_()  =>  { return ( unsafe { let _args_1: *const *const ()  = & ( * this_  ) . func_name  ; let mut _args_4 : *const ()  = ( unimplemented!()  as *const () ) ; format_string_9 ( c"filesystem error: in %s: %s%s [%s]" .as_ptr() , _args_1 , additional_msg , message , & mut _args_4 , ) } )   ;
 }, __v if __v ==  2_()  =>  { return ( unsafe { let _args_1: *const *const ()  = & ( * this_  ) . func_name  ; let mut _args_4 : *const ()  = ( unimplemented!()  as *const () ) ; let mut _args_5 : *const ()  = ( unimplemented!()  as *const () ) ; format_string_10 ( c"filesystem error: in %s: %s%s [%s] [%s]" .as_ptr() , _args_1 , additional_msg , message , & mut _args_4 , & mut _args_5 , ) } )   ;
 }, _ =>  { ( unsafe { test_fail_2 ( c"/app/cpp2rust/tests/libcxx/support/filesystem_test_helper.h" .as_ptr() , 618_() , & std::mem::transmute(*b"unexpected case\0") , ) } )  ;
 return unimplemented!()  ;
 }, } } ;
 panic!("ub: non-void function does not return a value") } )  .call ( )
        });
        unimplemented!();
        if !(unimplemented!()) {
            (unsafe {
                libc::fprintf(
                    libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
                    c"filesystem_error::what() does not match expected output:\n".as_ptr()
                        as *const libc::c_char,
                )
            });
            (unsafe {
                libc::fprintf(
                    libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
                    c"  expected: \"%s\"\n".as_ptr() as *const libc::c_char,
                    (unimplemented!()),
                )
            });
            (unsafe {
                libc::fprintf(
                    libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
                    c"  actual:   \"%s\"\n\n".as_ptr() as *const libc::c_char,
                    (unimplemented!()),
                )
            });
        }
    }
}
impl Default for ExceptionChecker {
    fn default() -> Self {
        unsafe {
            ExceptionChecker {
                expected_err: unimplemented!(),
                expected_path1: unimplemented!(),
                expected_path2: unimplemented!(),
                num_paths: unimplemented!(),
                func_name: std::ptr::null(),
                opt_message: unimplemented!(),
            }
        }
    }
}
pub unsafe fn GetWindowsInaccessibleDir_29() -> () {
    let dir: () = unimplemented!();
    let mut ec: () = unimplemented!();
    let root: () = unimplemented!();
    let mut __tmp_0: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_0;
    'loop_: for ent in IterRange::new(unimplemented!(), unimplemented!()) {
        let ent: *const () = &(*unimplemented!());
        if !(unimplemented!()) {
            continue 'loop_;
        }
        if (!(unimplemented!())) || (!(unimplemented!())) {
            (unsafe {
                libc:: fprintf ( libcc2rs::stderr_unsafe() as *mut ::libc::FILE , c"The expected inaccessible directory \"%s\" was found but doesn\'t behave as expected, skipping tests regarding it\n" .as_ptr() as *const libc::c_char , ( unimplemented!()  ) , )
            });
            return unimplemented!();
        }
        &(unimplemented!());
        if !(unimplemented!()) {
            (unsafe {
                libc:: fprintf ( libcc2rs::stderr_unsafe() as *mut ::libc::FILE , c"The expected inaccessible directory \"%s\" was found but seems to be accessible, skipping tests regarding it\n" .as_ptr() as *const libc::c_char , ( unimplemented!()  ) , )
            });
            return unimplemented!();
        }
        return unimplemented!();
    }
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            c"No inaccessible directory \"%s\" found, skipping tests regarding it\n".as_ptr()
                as *const libc::c_char,
            (unimplemented!()),
        )
    });
    return unimplemented!();
}
pub unsafe fn PathReserve_30(p: *mut (), mut N: ()) {
    let native_ref: *const () = unimplemented!();
    unimplemented!();
}
pub unsafe fn PathEq_31(LHS: *const (), RHS: *const ()) -> () {
    return unimplemented!();
}
pub unsafe fn PathEqIgnoreSep_32(mut LHS: (), mut RHS: ()) -> () {
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub static mut PathList_33: std::cell::LazyCell<[MultiStringType; 42]> = std::cell::LazyCell::new(
    || unsafe {
        [
            MultiStringType::new(
                { [0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 1]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 1]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 1]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 1]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"".as_ptr() },
                { (::std::mem::size_of::<[(); 1]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [32 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [32 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [32 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [32 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c" ".as_ptr() },
                { (::std::mem::size_of::<[(); 2]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [47 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"//".as_ptr() },
                { (::std::mem::size_of::<[(); 3]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c".".as_ptr() },
                { (::std::mem::size_of::<[(); 2]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [46 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"..".as_ptr() },
                { (::std::mem::size_of::<[(); 3]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"foo".as_ptr() },
                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"/".as_ptr() },
                { (::std::mem::size_of::<[(); 2]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [47 as (), 102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"/foo".as_ptr() },
                { (::std::mem::size_of::<[(); 5]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [102 as (), 111 as (), 111 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [102 as (), 111 as (), 111 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [102 as (), 111 as (), 111 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [102 as (), 111 as (), 111 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"foo/".as_ptr() },
                { (::std::mem::size_of::<[(); 5]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"/foo/".as_ptr() },
                { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (), 114 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (), 114 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (), 114 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (), 114 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"foo/bar".as_ptr() },
                { (::std::mem::size_of::<[(); 8]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (),
                        114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 9]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (),
                        114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 9]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (),
                        114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 9]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (),
                        114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 9]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"/foo/bar".as_ptr() },
                { (::std::mem::size_of::<[(); 9]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"//net".as_ptr() },
                { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 47 as (), 102 as (),
                        111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 47 as (), 102 as (),
                        111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 47 as (), 102 as (),
                        111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 47 as (), 102 as (),
                        111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"//net/foo".as_ptr() },
                { (::std::mem::size_of::<[(); 10]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        47 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        47 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        47 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        47 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"///foo///".as_ptr() },
                { (::std::mem::size_of::<[(); 10]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        47 as (), 47 as (), 98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 13]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        47 as (), 47 as (), 98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 13]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        47 as (), 47 as (), 98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 13]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        47 as (), 47 as (), 98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 13]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"///foo///bar".as_ptr() },
                { (::std::mem::size_of::<[(); 13]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [47 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"/.".as_ptr() },
                { (::std::mem::size_of::<[(); 3]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [46 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"./".as_ptr() },
                { (::std::mem::size_of::<[(); 3]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [47 as (), 46 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 46 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 46 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 46 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"/..".as_ptr() },
                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [46 as (), 46 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 46 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 46 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 46 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"../".as_ptr() },
                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"foo/.".as_ptr() },
                { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"foo/..".as_ptr() },
                { (::std::mem::size_of::<[(); 7]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"foo/./".as_ptr() },
                { (::std::mem::size_of::<[(); 7]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 98 as (),
                        97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 98 as (),
                        97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 98 as (),
                        97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 98 as (),
                        97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"foo/./bar".as_ptr() },
                { (::std::mem::size_of::<[(); 10]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"foo/../".as_ptr() },
                { (::std::mem::size_of::<[(); 8]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"foo/../bar".as_ptr() },
                { (::std::mem::size_of::<[(); 11]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [99 as (), 58 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:".as_ptr() },
                { (::std::mem::size_of::<[(); 3]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [99 as (), 58 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:/".as_ptr() },
                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:foo".as_ptr() },
                { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:/foo".as_ptr() },
                { (::std::mem::size_of::<[(); 7]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:foo/".as_ptr() },
                { (::std::mem::size_of::<[(); 7]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:/foo/".as_ptr() },
                { (::std::mem::size_of::<[(); 8]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:/foo/bar".as_ptr() },
                { (::std::mem::size_of::<[(); 11]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [112 as (), 114 as (), 110 as (), 58 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [112 as (), 114 as (), 110 as (), 58 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [112 as (), 114 as (), 110 as (), 58 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [112 as (), 114 as (), 110 as (), 58 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"prn:".as_ptr() },
                { (::std::mem::size_of::<[(); 5]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [99 as (), 58 as (), 92 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 92 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 92 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 92 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:\\".as_ptr() },
                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:\\foo".as_ptr() },
                { (::std::mem::size_of::<[(); 7]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 92 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 92 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 92 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 92 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:foo\\".as_ptr() },
                { (::std::mem::size_of::<[(); 7]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:\\foo\\".as_ptr() },
                { (::std::mem::size_of::<[(); 8]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:\\foo/".as_ptr() },
                { (::std::mem::size_of::<[(); 8]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:/foo\\bar".as_ptr() },
                { (::std::mem::size_of::<[(); 11]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [47 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"//".as_ptr() },
                { (::std::mem::size_of::<[(); 3]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        47 as (), 102 as (), 105 as (), 110 as (), 97 as (), 108 as (), 108 as (),
                        121 as (), 47 as (), 119 as (), 101 as (), 47 as (), 110 as (), 101 as (),
                        101 as (), 100 as (), 47 as (), 111 as (), 110 as (), 101 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 47 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 82]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 102 as (), 105 as (), 110 as (), 97 as (), 108 as (), 108 as (),
                        121 as (), 47 as (), 119 as (), 101 as (), 47 as (), 110 as (), 101 as (),
                        101 as (), 100 as (), 47 as (), 111 as (), 110 as (), 101 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 47 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 82]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 102 as (), 105 as (), 110 as (), 97 as (), 108 as (), 108 as (),
                        121 as (), 47 as (), 119 as (), 101 as (), 47 as (), 110 as (), 101 as (),
                        101 as (), 100 as (), 47 as (), 111 as (), 110 as (), 101 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 47 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 82]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 102 as (), 105 as (), 110 as (), 97 as (), 108 as (), 108 as (),
                        121 as (), 47 as (), 119 as (), 101 as (), 47 as (), 110 as (), 101 as (),
                        101 as (), 100 as (), 47 as (), 111 as (), 110 as (), 101 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 47 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 82]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    c"/finally/we/need/one/really/really/really/really/really/really/really/long/string" .as_ptr()
                },
                { (::std::mem::size_of::<[(); 82]>() as ()).wrapping_sub(1_()) },
            ),
        ]
    },
);
pub static mut PathListSize_34: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (::std::mem::size_of::<[MultiStringType; 42]>() as ())
        .wrapping_div((::std::mem::size_of::<MultiStringType>() as ()))
});
pub unsafe fn absolute_signature_test_35() {
    let p: () = unimplemented!();
    (&(p));
    let mut ec: () = unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn basic_test_36() {
    let cwd: () = unimplemented!();
    let TestCases: [anon_37; 4] = [
        anon_37 {
            input: unimplemented!(),
            expect: unimplemented!(),
        },
        anon_37 {
            input: unimplemented!(),
            expect: unimplemented!(),
        },
        anon_37 {
            input: unimplemented!(),
            expect: unimplemented!(),
        },
        anon_37 {
            input: unimplemented!(),
            expect: unimplemented!(),
        },
    ];
    'loop_: for TC in IterRange::new(
        TestCases.as_ptr(),
        TestCases.as_ptr().offset((4_()) as isize),
    ) {
        let TC: *const anon_37 = &(*TC);
        let mut ec: () = (unsafe { GetTestEC_26(None) });
        let ret: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct anon_37 {
    pub input: (),
    pub expect: (),
}
impl Default for anon_37 {
    fn default() -> Self {
        unsafe {
            anon_37 {
                input: unimplemented!(),
                expect: unimplemented!(),
            }
        }
    }
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { absolute_signature_test_35() });
    (unsafe { basic_test_36() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const PathList_33);
    std::cell::LazyCell::force(&*&raw const PathListSize_34);
}
