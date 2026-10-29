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
    unimplemented!();
}
pub unsafe fn test_eprintf_1(
    mut fmt: *const (),
    args_1: *const *const (),
    args_2: *const *const (),
    args_3: *const (),
) {
    unimplemented!();
}
pub unsafe fn test_eprintf_2(mut fmt: *const (), args: *const *const ()) {
    unimplemented!();
}
pub unsafe fn test_log_3(
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
pub unsafe fn test_log_4(
    mut condition: *const (),
    mut file: *const (),
    mut line: (),
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_fail_5(mut file: *const (), mut line: (), arg: *const [(); 16]) {
    (unsafe { test_log_3(c"".as_ptr(), file, line, (*arg).as_ptr()) });
    unimplemented!();
}
pub unsafe fn test_require_6(
    mut condition: (),
    mut condition_str: *const (),
    mut file: *const (),
    mut line: (),
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const () = condition_str;
        let _file: *const () = file;
        let _line: () = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_4(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_is_high_surrogate_7(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (56319_()));
}
pub unsafe fn test_is_low_surrogate_8(mut value: ()) -> () {
    return ((value) >= (56320_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_surrogate_9(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_code_point_10(mut value: ()) -> () {
    return ((value) <= (1114111_()));
}
pub unsafe fn test_is_scalar_value_11(mut value: ()) -> () {
    return (unsafe { test_is_code_point_10(value) }) && (!(unsafe { test_is_surrogate_9(value) }));
}
pub static mut test_replacement_character_12: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { (65533 as ()) });
pub unsafe fn operator_shl_13(os: *mut (), loc: *const ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_14(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_15(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_16(stream: *mut (), value: *const [(); 18]) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_17(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_18(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_message_19(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        let mut sstr: () = unimplemented!();
        {
            (unsafe {
                let _stream: *mut () = &mut sstr;
                let _value: *const [(); 18] = args_0;
                test_concat_16(_stream, _value)
            });
            {
                (unsafe {
                    let _stream: *mut () = &mut sstr;
                    let _value: *mut () = args_1;
                    test_concat_18(_stream, _value)
                });
                {
                    (unsafe {
                        let _stream: *mut () = &mut sstr;
                        let _value: *const [(); 18] = args_2;
                        test_concat_16(_stream, _value)
                    });
                    {
                        (unsafe {
                            let _stream: *mut () = &mut sstr;
                            let _value: *mut () = args_3;
                            test_concat_17(_stream, _value)
                        });
                        {
                            (unsafe {
                                let _stream: *mut () = &mut sstr;
                                let _value: *const [(); 18] = args_4;
                                test_concat_16(_stream, _value)
                            });
                            {
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_5;
                                    test_concat_15(_stream, _value)
                                });
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_6;
                                    test_concat_14(_stream, _value)
                                })
                            }
                        }
                    }
                }
            }
        };
        return unimplemented!();
    }
    panic!("ub: non-void function does not return a value")
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
pub unsafe fn format_string_imp_20(mut msg: *const (), __args: Box<[VaArg]>) -> () {
    let mut args: *const VaArg = ::std::ptr::null();
    args = __args.as_ptr();
    let mut args_guard: GuardVAList_21 = GuardVAList_21::new({ &mut args });
    let _dtor_args_guard =
        ScopedDestructorUnsafe::new(&raw mut args_guard, GuardVAList_21::destructor);
    let mut args_cp: *const VaArg = ::std::ptr::null();
    args_cp = args.clone();
    let mut args_copy_guard: GuardVAList_21 = GuardVAList_21::new({ &mut args_cp });
    let _dtor_args_copy_guard =
        ScopedDestructorUnsafe::new(&raw mut args_copy_guard, GuardVAList_21::destructor);
    let mut local_buff: () = unimplemented!();
    let mut size: () = unimplemented!();
    let mut ret: () = unimplemented!();
    (unsafe { GuardVAList_21::clear(&mut args_copy_guard) });
    if ((ret) == (0_())) {
        return unimplemented!();
    }
    if ((ret) < (size)) {
        return unimplemented!();
    }
    size = (ret).wrapping_add(1_());
    let mut buff_ptr: () = unimplemented!();
    ret = unimplemented!();
    return unimplemented!();
}
pub unsafe fn unwrap_22(s: *mut ()) -> *const () {
    return unimplemented!();
}
pub unsafe fn unwrap_23(a: *mut *const ()) -> *const *const () {
    const _: () = assert!(true, "!std::is_class<Arg>::value");;
    return a;
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct GuardVAList_21 {
    pub xtarget: *mut *const VaArg,
    pub active: (),
}
impl GuardVAList_21 {
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
pub unsafe fn format_string_24(
    mut fmt: *const (),
    args_1: *const *const (),
    args_2: *const (),
    args_3: *const (),
) -> () {
    return (unsafe {
        format_string_imp_20(
            fmt,
            Box::new([
                (*(unsafe { unwrap_23((args_1).cast_mut()) })).into(),
                (unsafe { unwrap_22((args_2).cast_mut()) }).into(),
                (unsafe { unwrap_22((args_3).cast_mut()) }).into(),
            ]),
        )
    });
}
pub unsafe fn format_string_25(
    mut fmt: *const (),
    args_1: *const *const (),
    args_2: *const (),
    args_3: *const (),
    args_4: *const *const (),
) -> () {
    return (unsafe {
        format_string_imp_20(
            fmt,
            Box::new([
                (*(unsafe { unwrap_23((args_1).cast_mut()) })).into(),
                (unsafe { unwrap_22((args_2).cast_mut()) }).into(),
                (unsafe { unwrap_22((args_3).cast_mut()) }).into(),
                (*(unsafe { unwrap_23((args_4).cast_mut()) })).into(),
            ]),
        )
    });
}
pub unsafe fn format_string_26(
    mut fmt: *const (),
    args_1: *const *const (),
    args_2: *const (),
    args_3: *const (),
    args_4: *const *const (),
    args_5: *const *const (),
) -> () {
    return (unsafe {
        format_string_imp_20(
            fmt,
            Box::new([
                (*(unsafe { unwrap_23((args_1).cast_mut()) })).into(),
                (unsafe { unwrap_22((args_2).cast_mut()) }).into(),
                (unsafe { unwrap_22((args_3).cast_mut()) }).into(),
                (*(unsafe { unwrap_23((args_4).cast_mut()) })).into(),
                (*(unsafe { unwrap_23((args_5).cast_mut()) })).into(),
            ]),
        )
    });
}
pub unsafe fn symlink_27(mut oldname: *const (), mut newname: *const (), mut is_dir: ()) -> () {
    &(is_dir);
    return unimplemented!();
}
pub unsafe fn space_28(mut path: (), capacity: *mut (), free: *mut (), avail: *mut ()) -> () {
    let mut expect: () = unimplemented!();
    if ((unimplemented!()) == (-1_())) {
        return false;
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut do_mult: FnPtr<fn(()) -> ()> = lambda_unsafe!(
        {
            let expect: *mut () = &mut expect;
        },
        |mut val: ()| -> () {
            let mut fsize: () = (*expect).f_frsize;
            let mut new_val: () = (val).wrapping_mul(fsize);
            unimplemented!();
            return new_val;
        }
    );
    (*capacity) = (unsafe { do_mult.call(expect.f_blocks) });
    (*free) = (unsafe { do_mult.call(expect.f_bfree) });
    (*avail) = (unsafe { do_mult.call(expect.f_bavail) });
    return true;
}
pub unsafe fn fopen64_29(mut pathname: *const (), mut mode: *const ()) -> () {
    return unimplemented!();
}
pub unsafe fn ftruncate64_30(mut fd: (), mut length: ()) -> () {
    return unimplemented!();
}
pub unsafe fn getcwd_31() -> () {
    let mut buf: [(); 4096] = [unimplemented!(); 4096];
    let mut ret: *mut () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn exists_32(path: *const ()) -> () {
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
        let mut ret: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        this
    }
    pub unsafe fn destructor(&mut self) {
        let mut cmd: () = unimplemented!();
        let mut ret: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        ret = unimplemented!();
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
    pub unsafe fn create_file(&mut self, mut filename_path: (), mut size: Option<()>) -> () {
        let mut size: () = size.unwrap_or_else(|| unsafe { 0_() });
        let mut filename: () = (unsafe { scoped_test_env::sanitize_path(self, unimplemented!()) });
        if ((size) > (unimplemented!())) {
            unimplemented!();
            unimplemented!();
        }
        let mut file: () = (unsafe { fopen64_29(unimplemented!(), c"we".as_ptr()) });
        if (file).is_null() {
            unimplemented!();
            unimplemented!();
        }
        if ((unsafe { ftruncate64_30(unimplemented!(), size) }) == (-1_())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
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
            symlink_27(_oldname, _newname, is_dir)
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
        let mut ret: () = unimplemented!();
        unimplemented!();
        return unimplemented!();
    }
    pub unsafe fn create_fifo(&mut self, mut file: ()) -> () {
        unimplemented!();
        let mut ret: () = unimplemented!();
        unimplemented!();
        return unimplemented!();
    }
    pub unsafe fn create_socket(&mut self, mut file: ()) -> () {
        unimplemented!();
        let mut address: () = unimplemented!();
        address.sun_family = 1_();
        if ((unimplemented!()) > (::std::mem::size_of::<[(); 108]>())) {
            unimplemented!();
        }
        unimplemented!();
        unimplemented!();
        let mut fd: () = unimplemented!();
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
            exists_32(&mut _path)
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
        (unsafe { scoped_test_env::create_file(&mut this.env_, unimplemented!(), Some(42_())) });
        (unsafe { scoped_test_env::create_file(&mut this.env_, unimplemented!(), None) });
        (unsafe { scoped_test_env::create_file(&mut this.env_, unimplemented!(), Some(42_())) });
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
            oldCwd_: (unsafe { getcwd_31() }),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        let mut ret: () = unimplemented!();
        unimplemented!();
    }
}
impl Default for CWDGuard {
    fn default() -> Self {
        unsafe { CWDGuard::new() }
    }
}
pub unsafe fn GetTestEC_33(mut Idx: Option<()>) -> () {
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
pub unsafe fn ErrorIsImp_34(ec: *const (), errors: *const ()) -> () {
    let mut cond: () = unimplemented!();
    'loop_: for errc in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut errc: () = (*unimplemented!());
        if ((unimplemented!()) == (errc as ())) {
            return true;
        }
    }
    return false;
}
pub unsafe fn NormalizeExpectedPerms_35(mut P: ()) -> () {
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
        const _: () = assert!(true, "");;
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
 } , | | -> () { 'switch: { match { ( * this_  ) . num_paths   } { __v if __v ==  0_()  =>  { return ( unsafe { format_string_24 ( c"filesystem error: in %s: %s%s" .as_ptr() , & ( * this_  ) . func_name  , additional_msg , message , ) } )   ;
 }, __v if __v ==  1_()  =>  { return ( unsafe { let _args_1: *const *const ()  = & ( * this_  ) . func_name  ; let mut _args_4 : *const ()  = ( unimplemented!()  as *const () ) ; format_string_25 ( c"filesystem error: in %s: %s%s [%s]" .as_ptr() , _args_1 , additional_msg , message , & mut _args_4 , ) } )   ;
 }, __v if __v ==  2_()  =>  { return ( unsafe { let _args_1: *const *const ()  = & ( * this_  ) . func_name  ; let mut _args_4 : *const ()  = ( unimplemented!()  as *const () ) ; let mut _args_5 : *const ()  = ( unimplemented!()  as *const () ) ; format_string_26 ( c"filesystem error: in %s: %s%s [%s] [%s]" .as_ptr() , _args_1 , additional_msg , message , & mut _args_4 , & mut _args_5 , ) } )   ;
 }, _ =>  { ( unsafe { test_fail_5 ( c"/app/cpp2rust/tests/libcxx/support/filesystem_test_helper.h" .as_ptr() , 618_() , & std::mem::transmute(*b"unexpected case\0") , ) } )  ;
 return unimplemented!()  ;
 }, } } ;
 panic!("ub: non-void function does not return a value") } )  .call ( )
        });
        unimplemented!();
        if !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
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
pub unsafe fn GetWindowsInaccessibleDir_36() -> () {
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
            unimplemented!();
            return unimplemented!();
        }
        &(unimplemented!());
        if !(unimplemented!()) {
            unimplemented!();
            return unimplemented!();
        }
        return unimplemented!();
    }
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn print_tests_37(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe {
        check
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        check
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        let mut _args: () = (' ' as ());
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(unimplemented!(), unimplemented!(), &mut _args)
    });
    (unsafe {
        check
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        let mut _args: () = (' ' as ());
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(unimplemented!(), unimplemented!(), &mut _args)
    });
    (unsafe {
        let mut _args_2: () = false;
        let mut _args_3: () = true;
        check.spec::<fn((), (), *mut (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut _args_2,
            &mut _args_3,
        )
    });
    (unsafe {
        let mut _args_2: () = false;
        let mut _args_3: () = true;
        check.spec::<fn((), (), *mut (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut _args_2,
            &mut _args_3,
        )
    });
    (unsafe {
        let mut _args_2: () = 1_();
        let mut _args_3: () = 2_();
        let mut _args_4: () = 3_();
        let mut _args_5: () = 4_();
        let mut _args_6: () = 5_();
        let mut _args_7: () = 6_();
        let mut _args_8: () = 7_();
        let mut _args_9: () = 8_();
        let mut _args_10: () = 9_();
        let mut _args_11: () = 0_();
        let mut _args_12: () = 1_();
        let mut _args_13: () = 2_();
        let mut _args_14: () = 3_();
        let mut _args_15: () = 4_();
        let mut _args_16: () = 5_();
        let mut _args_17: () = 6_();
        let mut _args_18: () = 7_();
        let mut _args_19: () = 8_();
        let mut _args_20: () = 9_();
        let mut _args_21: () = 0_();
        check
            .spec::<fn(
                (),
                (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
            )>(3)
            .call(
                unimplemented!(),
                unimplemented!(),
                &mut _args_2,
                &mut _args_3,
                &mut _args_4,
                &mut _args_5,
                &mut _args_6,
                &mut _args_7,
                &mut _args_8,
                &mut _args_9,
                &mut _args_10,
                &mut _args_11,
                &mut _args_12,
                &mut _args_13,
                &mut _args_14,
                &mut _args_15,
                &mut _args_16,
                &mut _args_17,
                &mut _args_18,
                &mut _args_19,
                &mut _args_20,
                &mut _args_21,
            )
    });
    (unsafe {
        let mut _args_2: () = ('\0' as ());
        check.spec::<fn((), (), *mut (), *const [(); 6])>(4).call(
            unimplemented!(),
            unimplemented!(),
            &mut _args_2,
            &std::mem::transmute(*b"world\0"),
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check.spec::<fn((), (), *const [(); 6])>(5).call(
            _expected,
            _fmt,
            &std::mem::transmute(*b"world\0"),
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, unimplemented!(), &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check.spec::<fn((), (), *const [(); 3])>(7).call(
            _expected,
            _fmt,
            &std::mem::transmute(*b"\xc2\xa1\0"),
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check.spec::<fn((), (), *const [(); 3])>(7).call(
            _expected,
            _fmt,
            &std::mem::transmute(*b"\xdf\xbf\0"),
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check.spec::<fn((), (), *const [(); 4])>(8).call(
            _expected,
            _fmt,
            &std::mem::transmute(*b"\xe0\xa0\x80\0"),
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check.spec::<fn((), (), *const [(); 4])>(8).call(
            _expected,
            _fmt,
            &std::mem::transmute(*b"\xef\xbf\xbd\0"),
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check.spec::<fn((), (), *const [(); 5])>(9).call(
            _expected,
            _fmt,
            &std::mem::transmute(*b"\xf0\x90\x80\x80\0"),
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check.spec::<fn((), (), *const [(); 5])>(9).call(
            _expected,
            _fmt,
            &std::mem::transmute(*b"\xf4\x8f\xbf\xbf\0"),
        )
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        let mut _arg2: () = 42_();
        check_exception.spec::<fn((), (), *mut ())>(1).call(
            unimplemented!(),
            unimplemented!(),
            &mut _arg2,
        )
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        let mut _arg2: () = 42_();
        check_exception.spec::<fn((), (), *mut ())>(1).call(
            unimplemented!(),
            unimplemented!(),
            &mut _arg2,
        )
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        let mut _arg2: () = 42_();
        check_exception.spec::<fn((), (), *mut ())>(1).call(
            unimplemented!(),
            unimplemented!(),
            &mut _arg2,
        )
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        let mut _arg2: () = 42_();
        check_exception.spec::<fn((), (), *mut ())>(1).call(
            unimplemented!(),
            unimplemented!(),
            &mut _arg2,
        )
    });
}
pub static mut env_38: std::cell::LazyCell<scoped_test_env> =
    std::cell::LazyCell::new(|| unsafe { scoped_test_env::new() });
pub static mut filename_39: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (unsafe {
        scoped_test_env::create_file(
            &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut env_38)),
            unimplemented!(),
            None,
        )
    })
});
pub static mut test_file_40: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe {
        lambda_unsafe!(
            Generic,
            {},
            |mut expected: (), mut fmt: ()| {
                let mut file: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                let mut stream: () = unimplemented!();
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_2(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_6(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"print.file.pass.cpp".as_ptr(),
                        60_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut file: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                let mut stream: () = unimplemented!();
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_2(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_41(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"print.file.pass.cpp".as_ptr(),
                        60_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut file: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                let mut stream: () = unimplemented!();
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_2(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_42(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"print.file.pass.cpp".as_ptr(),
                        60_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (),
             mut fmt: (),
             args_2: *mut (),
             args_3: *mut (),
             args_4: *mut (),
             args_5: *mut (),
             args_6: *mut (),
             args_7: *mut (),
             args_8: *mut (),
             args_9: *mut (),
             args_10: *mut (),
             args_11: *mut (),
             args_12: *mut (),
             args_13: *mut (),
             args_14: *mut (),
             args_15: *mut (),
             args_16: *mut (),
             args_17: *mut (),
             args_18: *mut (),
             args_19: *mut (),
             args_20: *mut (),
             args_21: *mut ()| {
                let mut file: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                let mut stream: () = unimplemented!();
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_2(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_43(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"print.file.pass.cpp".as_ptr(),
                        60_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *const [(); 6]| {
                let mut file: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                let mut stream: () = unimplemented!();
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_2(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_44(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"print.file.pass.cpp".as_ptr(),
                        60_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *const [(); 6]| {
                let mut file: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                let mut stream: () = unimplemented!();
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_2(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_45(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"print.file.pass.cpp".as_ptr(),
                        60_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut file: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                let mut stream: () = unimplemented!();
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_2(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_46(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"print.file.pass.cpp".as_ptr(),
                        60_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *const [(); 3]| {
                let mut file: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                let mut stream: () = unimplemented!();
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_2(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_47(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"print.file.pass.cpp".as_ptr(),
                        60_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *const [(); 4]| {
                let mut file: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                let mut stream: () = unimplemented!();
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_2(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_48(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"print.file.pass.cpp".as_ptr(),
                        60_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *const [(); 5]| {
                let mut file: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                let mut stream: () = unimplemented!();
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_2(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_49(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"print.file.pass.cpp".as_ptr(),
                        60_(),
                        &mut _arg,
                    )
                });
            }
        )
    });
pub static mut test_exception_50: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe {
        lambda_unsafe!(Generic, {}, |mut _a0: (), mut _a1: ()| {}, |mut _a0: (),
                                                                    mut _a1: (),
                                                                    _a2: *mut (
        )| {})
    });
pub unsafe fn test_wide_stream_51() {
    let mut file: () = unimplemented!();
    unimplemented!();
    let mut mode: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_read_only_52() {
    let mut file: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_new_line_53() {
    {
        let mut file: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut file: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
    (unsafe {
        print_tests_37(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_file_40)).copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_exception_50)).copy_from(),
        )
    });
    (unsafe { test_wide_stream_51() });
    (unsafe { test_read_only_52() });
    (unsafe { test_new_line_53() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const test_replacement_character_12);
    std::cell::LazyCell::force(&*&raw const env_38);
    std::cell::LazyCell::force(&*&raw const filename_39);
    std::cell::LazyCell::force(&*&raw const test_file_40);
    std::cell::LazyCell::force(&*&raw const test_exception_50);
}
