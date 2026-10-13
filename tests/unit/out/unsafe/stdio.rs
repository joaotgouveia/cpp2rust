extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct __fsid_t {
    pub __val: [i32; 2],
}
impl Clone for __fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __mbstate_t {
    pub __count: i32,
    pub __value: anon_0,
}
impl Clone for __mbstate_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __mbstate_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct anon_0 {
    pub __wch: u32,
    pub __wchb: [libc::c_char; 4],
}
impl Clone for anon_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _G_fpos_t {
    pub __pos: i64,
    pub __state: __mbstate_t,
}
impl Clone for _G_fpos_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _G_fpos_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _G_fpos64_t {
    pub __pos: i64,
    pub __state: __mbstate_t,
}
impl Clone for _G_fpos64_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _G_fpos64_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: i64,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut ::libc::c_void,
    pub _offset: i64,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::libc::c_void,
    pub __pad5: usize,
    pub _mode: i32,
    pub _unused2: [libc::c_char; 20],
}
impl Clone for _IO_FILE {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _IO_FILE {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _IO_cookie_io_functions_t {
    pub read: Option<unsafe fn(*mut ::libc::c_void, *mut libc::c_char, usize) -> i64>,
    pub write: Option<unsafe fn(*mut ::libc::c_void, *const libc::c_char, usize) -> i64>,
    pub seek: Option<unsafe fn(*mut ::libc::c_void, *mut i64, i32) -> i32>,
    pub close: Option<unsafe fn(*mut ::libc::c_void) -> i32>,
}
impl Clone for _IO_cookie_io_functions_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _IO_cookie_io_functions_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut stdin_1: std::cell::LazyCell<*mut _IO_FILE> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut stdout_2: std::cell::LazyCell<*mut _IO_FILE> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut stderr_3: std::cell::LazyCell<*mut _IO_FILE> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub type anon_4 = u32;
pub const anon_4__SC_ARG_MAX: anon_4 = 0;
pub const anon_4__SC_CHILD_MAX: anon_4 = 1;
pub const anon_4__SC_CLK_TCK: anon_4 = 2;
pub const anon_4__SC_NGROUPS_MAX: anon_4 = 3;
pub const anon_4__SC_OPEN_MAX: anon_4 = 4;
pub const anon_4__SC_STREAM_MAX: anon_4 = 5;
pub const anon_4__SC_TZNAME_MAX: anon_4 = 6;
pub const anon_4__SC_JOB_CONTROL: anon_4 = 7;
pub const anon_4__SC_SAVED_IDS: anon_4 = 8;
pub const anon_4__SC_REALTIME_SIGNALS: anon_4 = 9;
pub const anon_4__SC_PRIORITY_SCHEDULING: anon_4 = 10;
pub const anon_4__SC_TIMERS: anon_4 = 11;
pub const anon_4__SC_ASYNCHRONOUS_IO: anon_4 = 12;
pub const anon_4__SC_PRIORITIZED_IO: anon_4 = 13;
pub const anon_4__SC_SYNCHRONIZED_IO: anon_4 = 14;
pub const anon_4__SC_FSYNC: anon_4 = 15;
pub const anon_4__SC_MAPPED_FILES: anon_4 = 16;
pub const anon_4__SC_MEMLOCK: anon_4 = 17;
pub const anon_4__SC_MEMLOCK_RANGE: anon_4 = 18;
pub const anon_4__SC_MEMORY_PROTECTION: anon_4 = 19;
pub const anon_4__SC_MESSAGE_PASSING: anon_4 = 20;
pub const anon_4__SC_SEMAPHORES: anon_4 = 21;
pub const anon_4__SC_SHARED_MEMORY_OBJECTS: anon_4 = 22;
pub const anon_4__SC_AIO_LISTIO_MAX: anon_4 = 23;
pub const anon_4__SC_AIO_MAX: anon_4 = 24;
pub const anon_4__SC_AIO_PRIO_DELTA_MAX: anon_4 = 25;
pub const anon_4__SC_DELAYTIMER_MAX: anon_4 = 26;
pub const anon_4__SC_MQ_OPEN_MAX: anon_4 = 27;
pub const anon_4__SC_MQ_PRIO_MAX: anon_4 = 28;
pub const anon_4__SC_VERSION: anon_4 = 29;
pub const anon_4__SC_PAGESIZE: anon_4 = 30;
pub const anon_4__SC_RTSIG_MAX: anon_4 = 31;
pub const anon_4__SC_SEM_NSEMS_MAX: anon_4 = 32;
pub const anon_4__SC_SEM_VALUE_MAX: anon_4 = 33;
pub const anon_4__SC_SIGQUEUE_MAX: anon_4 = 34;
pub const anon_4__SC_TIMER_MAX: anon_4 = 35;
pub const anon_4__SC_BC_BASE_MAX: anon_4 = 36;
pub const anon_4__SC_BC_DIM_MAX: anon_4 = 37;
pub const anon_4__SC_BC_SCALE_MAX: anon_4 = 38;
pub const anon_4__SC_BC_STRING_MAX: anon_4 = 39;
pub const anon_4__SC_COLL_WEIGHTS_MAX: anon_4 = 40;
pub const anon_4__SC_EQUIV_CLASS_MAX: anon_4 = 41;
pub const anon_4__SC_EXPR_NEST_MAX: anon_4 = 42;
pub const anon_4__SC_LINE_MAX: anon_4 = 43;
pub const anon_4__SC_RE_DUP_MAX: anon_4 = 44;
pub const anon_4__SC_CHARCLASS_NAME_MAX: anon_4 = 45;
pub const anon_4__SC_2_VERSION: anon_4 = 46;
pub const anon_4__SC_2_C_BIND: anon_4 = 47;
pub const anon_4__SC_2_C_DEV: anon_4 = 48;
pub const anon_4__SC_2_FORT_DEV: anon_4 = 49;
pub const anon_4__SC_2_FORT_RUN: anon_4 = 50;
pub const anon_4__SC_2_SW_DEV: anon_4 = 51;
pub const anon_4__SC_2_LOCALEDEF: anon_4 = 52;
pub const anon_4__SC_PII: anon_4 = 53;
pub const anon_4__SC_PII_XTI: anon_4 = 54;
pub const anon_4__SC_PII_SOCKET: anon_4 = 55;
pub const anon_4__SC_PII_INTERNET: anon_4 = 56;
pub const anon_4__SC_PII_OSI: anon_4 = 57;
pub const anon_4__SC_POLL: anon_4 = 58;
pub const anon_4__SC_SELECT: anon_4 = 59;
pub const anon_4__SC_UIO_MAXIOV: anon_4 = 60;
pub const anon_4__SC_IOV_MAX: anon_4 = 60;
pub const anon_4__SC_PII_INTERNET_STREAM: anon_4 = 61;
pub const anon_4__SC_PII_INTERNET_DGRAM: anon_4 = 62;
pub const anon_4__SC_PII_OSI_COTS: anon_4 = 63;
pub const anon_4__SC_PII_OSI_CLTS: anon_4 = 64;
pub const anon_4__SC_PII_OSI_M: anon_4 = 65;
pub const anon_4__SC_T_IOV_MAX: anon_4 = 66;
pub const anon_4__SC_THREADS: anon_4 = 67;
pub const anon_4__SC_THREAD_SAFE_FUNCTIONS: anon_4 = 68;
pub const anon_4__SC_GETGR_R_SIZE_MAX: anon_4 = 69;
pub const anon_4__SC_GETPW_R_SIZE_MAX: anon_4 = 70;
pub const anon_4__SC_LOGIN_NAME_MAX: anon_4 = 71;
pub const anon_4__SC_TTY_NAME_MAX: anon_4 = 72;
pub const anon_4__SC_THREAD_DESTRUCTOR_ITERATIONS: anon_4 = 73;
pub const anon_4__SC_THREAD_KEYS_MAX: anon_4 = 74;
pub const anon_4__SC_THREAD_STACK_MIN: anon_4 = 75;
pub const anon_4__SC_THREAD_THREADS_MAX: anon_4 = 76;
pub const anon_4__SC_THREAD_ATTR_STACKADDR: anon_4 = 77;
pub const anon_4__SC_THREAD_ATTR_STACKSIZE: anon_4 = 78;
pub const anon_4__SC_THREAD_PRIORITY_SCHEDULING: anon_4 = 79;
pub const anon_4__SC_THREAD_PRIO_INHERIT: anon_4 = 80;
pub const anon_4__SC_THREAD_PRIO_PROTECT: anon_4 = 81;
pub const anon_4__SC_THREAD_PROCESS_SHARED: anon_4 = 82;
pub const anon_4__SC_NPROCESSORS_CONF: anon_4 = 83;
pub const anon_4__SC_NPROCESSORS_ONLN: anon_4 = 84;
pub const anon_4__SC_PHYS_PAGES: anon_4 = 85;
pub const anon_4__SC_AVPHYS_PAGES: anon_4 = 86;
pub const anon_4__SC_ATEXIT_MAX: anon_4 = 87;
pub const anon_4__SC_PASS_MAX: anon_4 = 88;
pub const anon_4__SC_XOPEN_VERSION: anon_4 = 89;
pub const anon_4__SC_XOPEN_XCU_VERSION: anon_4 = 90;
pub const anon_4__SC_XOPEN_UNIX: anon_4 = 91;
pub const anon_4__SC_XOPEN_CRYPT: anon_4 = 92;
pub const anon_4__SC_XOPEN_ENH_I18N: anon_4 = 93;
pub const anon_4__SC_XOPEN_SHM: anon_4 = 94;
pub const anon_4__SC_2_CHAR_TERM: anon_4 = 95;
pub const anon_4__SC_2_C_VERSION: anon_4 = 96;
pub const anon_4__SC_2_UPE: anon_4 = 97;
pub const anon_4__SC_XOPEN_XPG2: anon_4 = 98;
pub const anon_4__SC_XOPEN_XPG3: anon_4 = 99;
pub const anon_4__SC_XOPEN_XPG4: anon_4 = 100;
pub const anon_4__SC_CHAR_BIT: anon_4 = 101;
pub const anon_4__SC_CHAR_MAX: anon_4 = 102;
pub const anon_4__SC_CHAR_MIN: anon_4 = 103;
pub const anon_4__SC_INT_MAX: anon_4 = 104;
pub const anon_4__SC_INT_MIN: anon_4 = 105;
pub const anon_4__SC_LONG_BIT: anon_4 = 106;
pub const anon_4__SC_WORD_BIT: anon_4 = 107;
pub const anon_4__SC_MB_LEN_MAX: anon_4 = 108;
pub const anon_4__SC_NZERO: anon_4 = 109;
pub const anon_4__SC_SSIZE_MAX: anon_4 = 110;
pub const anon_4__SC_SCHAR_MAX: anon_4 = 111;
pub const anon_4__SC_SCHAR_MIN: anon_4 = 112;
pub const anon_4__SC_SHRT_MAX: anon_4 = 113;
pub const anon_4__SC_SHRT_MIN: anon_4 = 114;
pub const anon_4__SC_UCHAR_MAX: anon_4 = 115;
pub const anon_4__SC_UINT_MAX: anon_4 = 116;
pub const anon_4__SC_ULONG_MAX: anon_4 = 117;
pub const anon_4__SC_USHRT_MAX: anon_4 = 118;
pub const anon_4__SC_NL_ARGMAX: anon_4 = 119;
pub const anon_4__SC_NL_LANGMAX: anon_4 = 120;
pub const anon_4__SC_NL_MSGMAX: anon_4 = 121;
pub const anon_4__SC_NL_NMAX: anon_4 = 122;
pub const anon_4__SC_NL_SETMAX: anon_4 = 123;
pub const anon_4__SC_NL_TEXTMAX: anon_4 = 124;
pub const anon_4__SC_XBS5_ILP32_OFF32: anon_4 = 125;
pub const anon_4__SC_XBS5_ILP32_OFFBIG: anon_4 = 126;
pub const anon_4__SC_XBS5_LP64_OFF64: anon_4 = 127;
pub const anon_4__SC_XBS5_LPBIG_OFFBIG: anon_4 = 128;
pub const anon_4__SC_XOPEN_LEGACY: anon_4 = 129;
pub const anon_4__SC_XOPEN_REALTIME: anon_4 = 130;
pub const anon_4__SC_XOPEN_REALTIME_THREADS: anon_4 = 131;
pub const anon_4__SC_ADVISORY_INFO: anon_4 = 132;
pub const anon_4__SC_BARRIERS: anon_4 = 133;
pub const anon_4__SC_BASE: anon_4 = 134;
pub const anon_4__SC_C_LANG_SUPPORT: anon_4 = 135;
pub const anon_4__SC_C_LANG_SUPPORT_R: anon_4 = 136;
pub const anon_4__SC_CLOCK_SELECTION: anon_4 = 137;
pub const anon_4__SC_CPUTIME: anon_4 = 138;
pub const anon_4__SC_THREAD_CPUTIME: anon_4 = 139;
pub const anon_4__SC_DEVICE_IO: anon_4 = 140;
pub const anon_4__SC_DEVICE_SPECIFIC: anon_4 = 141;
pub const anon_4__SC_DEVICE_SPECIFIC_R: anon_4 = 142;
pub const anon_4__SC_FD_MGMT: anon_4 = 143;
pub const anon_4__SC_FIFO: anon_4 = 144;
pub const anon_4__SC_PIPE: anon_4 = 145;
pub const anon_4__SC_FILE_ATTRIBUTES: anon_4 = 146;
pub const anon_4__SC_FILE_LOCKING: anon_4 = 147;
pub const anon_4__SC_FILE_SYSTEM: anon_4 = 148;
pub const anon_4__SC_MONOTONIC_CLOCK: anon_4 = 149;
pub const anon_4__SC_MULTI_PROCESS: anon_4 = 150;
pub const anon_4__SC_SINGLE_PROCESS: anon_4 = 151;
pub const anon_4__SC_NETWORKING: anon_4 = 152;
pub const anon_4__SC_READER_WRITER_LOCKS: anon_4 = 153;
pub const anon_4__SC_SPIN_LOCKS: anon_4 = 154;
pub const anon_4__SC_REGEXP: anon_4 = 155;
pub const anon_4__SC_REGEX_VERSION: anon_4 = 156;
pub const anon_4__SC_SHELL: anon_4 = 157;
pub const anon_4__SC_SIGNALS: anon_4 = 158;
pub const anon_4__SC_SPAWN: anon_4 = 159;
pub const anon_4__SC_SPORADIC_SERVER: anon_4 = 160;
pub const anon_4__SC_THREAD_SPORADIC_SERVER: anon_4 = 161;
pub const anon_4__SC_SYSTEM_DATABASE: anon_4 = 162;
pub const anon_4__SC_SYSTEM_DATABASE_R: anon_4 = 163;
pub const anon_4__SC_TIMEOUTS: anon_4 = 164;
pub const anon_4__SC_TYPED_MEMORY_OBJECTS: anon_4 = 165;
pub const anon_4__SC_USER_GROUPS: anon_4 = 166;
pub const anon_4__SC_USER_GROUPS_R: anon_4 = 167;
pub const anon_4__SC_2_PBS: anon_4 = 168;
pub const anon_4__SC_2_PBS_ACCOUNTING: anon_4 = 169;
pub const anon_4__SC_2_PBS_LOCATE: anon_4 = 170;
pub const anon_4__SC_2_PBS_MESSAGE: anon_4 = 171;
pub const anon_4__SC_2_PBS_TRACK: anon_4 = 172;
pub const anon_4__SC_SYMLOOP_MAX: anon_4 = 173;
pub const anon_4__SC_STREAMS: anon_4 = 174;
pub const anon_4__SC_2_PBS_CHECKPOINT: anon_4 = 175;
pub const anon_4__SC_V6_ILP32_OFF32: anon_4 = 176;
pub const anon_4__SC_V6_ILP32_OFFBIG: anon_4 = 177;
pub const anon_4__SC_V6_LP64_OFF64: anon_4 = 178;
pub const anon_4__SC_V6_LPBIG_OFFBIG: anon_4 = 179;
pub const anon_4__SC_HOST_NAME_MAX: anon_4 = 180;
pub const anon_4__SC_TRACE: anon_4 = 181;
pub const anon_4__SC_TRACE_EVENT_FILTER: anon_4 = 182;
pub const anon_4__SC_TRACE_INHERIT: anon_4 = 183;
pub const anon_4__SC_TRACE_LOG: anon_4 = 184;
pub const anon_4__SC_LEVEL1_ICACHE_SIZE: anon_4 = 185;
pub const anon_4__SC_LEVEL1_ICACHE_ASSOC: anon_4 = 186;
pub const anon_4__SC_LEVEL1_ICACHE_LINESIZE: anon_4 = 187;
pub const anon_4__SC_LEVEL1_DCACHE_SIZE: anon_4 = 188;
pub const anon_4__SC_LEVEL1_DCACHE_ASSOC: anon_4 = 189;
pub const anon_4__SC_LEVEL1_DCACHE_LINESIZE: anon_4 = 190;
pub const anon_4__SC_LEVEL2_CACHE_SIZE: anon_4 = 191;
pub const anon_4__SC_LEVEL2_CACHE_ASSOC: anon_4 = 192;
pub const anon_4__SC_LEVEL2_CACHE_LINESIZE: anon_4 = 193;
pub const anon_4__SC_LEVEL3_CACHE_SIZE: anon_4 = 194;
pub const anon_4__SC_LEVEL3_CACHE_ASSOC: anon_4 = 195;
pub const anon_4__SC_LEVEL3_CACHE_LINESIZE: anon_4 = 196;
pub const anon_4__SC_LEVEL4_CACHE_SIZE: anon_4 = 197;
pub const anon_4__SC_LEVEL4_CACHE_ASSOC: anon_4 = 198;
pub const anon_4__SC_LEVEL4_CACHE_LINESIZE: anon_4 = 199;
pub const anon_4__SC_IPV6: anon_4 = 235;
pub const anon_4__SC_RAW_SOCKETS: anon_4 = 236;
pub const anon_4__SC_V7_ILP32_OFF32: anon_4 = 237;
pub const anon_4__SC_V7_ILP32_OFFBIG: anon_4 = 238;
pub const anon_4__SC_V7_LP64_OFF64: anon_4 = 239;
pub const anon_4__SC_V7_LPBIG_OFFBIG: anon_4 = 240;
pub const anon_4__SC_SS_REPL_MAX: anon_4 = 241;
pub const anon_4__SC_TRACE_EVENT_NAME_MAX: anon_4 = 242;
pub const anon_4__SC_TRACE_NAME_MAX: anon_4 = 243;
pub const anon_4__SC_TRACE_SYS_MAX: anon_4 = 244;
pub const anon_4__SC_TRACE_USER_EVENT_MAX: anon_4 = 245;
pub const anon_4__SC_XOPEN_STREAMS: anon_4 = 246;
pub const anon_4__SC_THREAD_ROBUST_PRIO_INHERIT: anon_4 = 247;
pub const anon_4__SC_THREAD_ROBUST_PRIO_PROTECT: anon_4 = 248;
pub const anon_4__SC_MINSIGSTKSZ: anon_4 = 249;
pub const anon_4__SC_SIGSTKSZ: anon_4 = 250;
pub unsafe fn test_fputc_5() {
    (unsafe {
        fputc_6(
            ('H' as i32),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut stdout_2)),
        )
    });
    (unsafe {
        fputc_6(
            ('i' as i32),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut stdout_2)),
        )
    });
    (unsafe {
        fputc_6(
            ('\n' as i32),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut stdout_2)),
        )
    });
}
pub unsafe fn test_fputs_7() {
    (unsafe {
        fputs_8(
            (c"hello".as_ptr().cast_mut()).cast_const(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut stdout_2)),
        )
    });
    (unsafe {
        fputc_6(
            ('\n' as i32),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut stdout_2)),
        )
    });
    let mut s: *const libc::c_char = (c"from variable".as_ptr().cast_mut()).cast_const();
    (unsafe {
        fputs_8(
            s,
            (*std::cell::LazyCell::force_mut(&mut *&raw mut stdout_2)),
        )
    });
    (unsafe {
        fputc_6(
            ('\n' as i32),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut stdout_2)),
        )
    });
    let mut buf: [libc::c_char; 4] = [
        (('b' as i32) as libc::c_char),
        (('u' as i32) as libc::c_char),
        (('f' as i32) as libc::c_char),
        (('\0' as i32) as libc::c_char),
    ];
    (unsafe {
        fputs_8(
            (buf.as_mut_ptr()).cast_const(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut stdout_2)),
        )
    });
    (unsafe {
        fputc_6(
            ('\n' as i32),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut stdout_2)),
        )
    });
}
pub unsafe fn test_puts_9() {
    (unsafe { puts_10((c"puts hello".as_ptr().cast_mut()).cast_const()) });
    let mut s: *const libc::c_char = (c"puts variable".as_ptr().cast_mut()).cast_const();
    (unsafe { puts_10(s) });
}
pub unsafe fn test_fileno_11() {
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fileno_12((*std::cell::LazyCell::force_mut(&mut *&raw mut stdin_1))) })
                == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_13(
                        (c"fileno(stdin) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio.c".as_ptr().cast_mut()).cast_const(),
                        29_u32,
                        c"void test_fileno(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                fileno_12((*std::cell::LazyCell::force_mut(&mut *&raw mut stdout_2)))
            }) == (1)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_13(
                        (c"fileno(stdout) == 1".as_ptr().cast_mut()).cast_const(),
                        (c"stdio.c".as_ptr().cast_mut()).cast_const(),
                        30_u32,
                        c"void test_fileno(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                fileno_12((*std::cell::LazyCell::force_mut(&mut *&raw mut stderr_3)))
            }) == (2)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_13(
                        (c"fileno(stderr) == 2".as_ptr().cast_mut()).cast_const(),
                        (c"stdio.c".as_ptr().cast_mut()).cast_const(),
                        31_u32,
                        c"void test_fileno(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut file: *const libc::c_char =
        (c"cpp2rust_fileno_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut _IO_FILE =
        (unsafe { fopen_14(file, (c"wb".as_ptr().cast_mut()).cast_const()) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((fp).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_13(
                        (c"fp != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"stdio.c".as_ptr().cast_mut()).cast_const(),
                        34_u32,
                        c"void test_fileno(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fileno_12(fp) }) > (2)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_13(
                        (c"fileno(fp) > 2".as_ptr().cast_mut()).cast_const(),
                        (c"stdio.c".as_ptr().cast_mut()).cast_const(),
                        35_u32,
                        c"void test_fileno(void)".as_ptr(),
                    )
                });
            }
        }
    };
    (unsafe { fclose_15(fp) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { unlink_16(file) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_13(
                        (c"unlink(file) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio.c".as_ptr().cast_mut()).cast_const(),
                        37_u32,
                        c"void test_fileno(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_fputc_5() });
    (unsafe { test_fputs_7() });
    (unsafe { test_puts_9() });
    (unsafe { test_fileno_11() });
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_wide_data;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_codecvt;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_marker;
pub unsafe fn fputc_6(mut __c: i32, mut __stream: *mut _IO_FILE) -> i32 {
    unimplemented!()
}
pub unsafe fn fputs_8(mut __s: *const libc::c_char, mut __stream: *mut _IO_FILE) -> i32 {
    unimplemented!()
}
pub unsafe fn puts_10(mut __s: *const libc::c_char) -> i32 {
    unimplemented!()
}
pub unsafe fn fileno_12(mut __stream: *mut _IO_FILE) -> i32 {
    unimplemented!()
}
pub unsafe fn __assert_fail_13(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn fopen_14(
    mut __filename: *const libc::c_char,
    mut __modes: *const libc::c_char,
) -> *mut _IO_FILE {
    unimplemented!()
}
pub unsafe fn fclose_15(mut __stream: *mut _IO_FILE) -> i32 {
    unimplemented!()
}
pub unsafe fn unlink_16(mut __name: *const libc::c_char) -> i32 {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
