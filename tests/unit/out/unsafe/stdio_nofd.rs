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
#[derive(VaArg, FnPtrArg)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const u16,
    pub __ctype_tolower: *const i32,
    pub __ctype_toupper: *const i32,
    pub __names: [*const libc::c_char; 13],
}
impl Clone for __locale_struct {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __locale_struct {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_1 = u32;
pub const anon_1__SC_ARG_MAX: anon_1 = 0;
pub const anon_1__SC_CHILD_MAX: anon_1 = 1;
pub const anon_1__SC_CLK_TCK: anon_1 = 2;
pub const anon_1__SC_NGROUPS_MAX: anon_1 = 3;
pub const anon_1__SC_OPEN_MAX: anon_1 = 4;
pub const anon_1__SC_STREAM_MAX: anon_1 = 5;
pub const anon_1__SC_TZNAME_MAX: anon_1 = 6;
pub const anon_1__SC_JOB_CONTROL: anon_1 = 7;
pub const anon_1__SC_SAVED_IDS: anon_1 = 8;
pub const anon_1__SC_REALTIME_SIGNALS: anon_1 = 9;
pub const anon_1__SC_PRIORITY_SCHEDULING: anon_1 = 10;
pub const anon_1__SC_TIMERS: anon_1 = 11;
pub const anon_1__SC_ASYNCHRONOUS_IO: anon_1 = 12;
pub const anon_1__SC_PRIORITIZED_IO: anon_1 = 13;
pub const anon_1__SC_SYNCHRONIZED_IO: anon_1 = 14;
pub const anon_1__SC_FSYNC: anon_1 = 15;
pub const anon_1__SC_MAPPED_FILES: anon_1 = 16;
pub const anon_1__SC_MEMLOCK: anon_1 = 17;
pub const anon_1__SC_MEMLOCK_RANGE: anon_1 = 18;
pub const anon_1__SC_MEMORY_PROTECTION: anon_1 = 19;
pub const anon_1__SC_MESSAGE_PASSING: anon_1 = 20;
pub const anon_1__SC_SEMAPHORES: anon_1 = 21;
pub const anon_1__SC_SHARED_MEMORY_OBJECTS: anon_1 = 22;
pub const anon_1__SC_AIO_LISTIO_MAX: anon_1 = 23;
pub const anon_1__SC_AIO_MAX: anon_1 = 24;
pub const anon_1__SC_AIO_PRIO_DELTA_MAX: anon_1 = 25;
pub const anon_1__SC_DELAYTIMER_MAX: anon_1 = 26;
pub const anon_1__SC_MQ_OPEN_MAX: anon_1 = 27;
pub const anon_1__SC_MQ_PRIO_MAX: anon_1 = 28;
pub const anon_1__SC_VERSION: anon_1 = 29;
pub const anon_1__SC_PAGESIZE: anon_1 = 30;
pub const anon_1__SC_RTSIG_MAX: anon_1 = 31;
pub const anon_1__SC_SEM_NSEMS_MAX: anon_1 = 32;
pub const anon_1__SC_SEM_VALUE_MAX: anon_1 = 33;
pub const anon_1__SC_SIGQUEUE_MAX: anon_1 = 34;
pub const anon_1__SC_TIMER_MAX: anon_1 = 35;
pub const anon_1__SC_BC_BASE_MAX: anon_1 = 36;
pub const anon_1__SC_BC_DIM_MAX: anon_1 = 37;
pub const anon_1__SC_BC_SCALE_MAX: anon_1 = 38;
pub const anon_1__SC_BC_STRING_MAX: anon_1 = 39;
pub const anon_1__SC_COLL_WEIGHTS_MAX: anon_1 = 40;
pub const anon_1__SC_EQUIV_CLASS_MAX: anon_1 = 41;
pub const anon_1__SC_EXPR_NEST_MAX: anon_1 = 42;
pub const anon_1__SC_LINE_MAX: anon_1 = 43;
pub const anon_1__SC_RE_DUP_MAX: anon_1 = 44;
pub const anon_1__SC_CHARCLASS_NAME_MAX: anon_1 = 45;
pub const anon_1__SC_2_VERSION: anon_1 = 46;
pub const anon_1__SC_2_C_BIND: anon_1 = 47;
pub const anon_1__SC_2_C_DEV: anon_1 = 48;
pub const anon_1__SC_2_FORT_DEV: anon_1 = 49;
pub const anon_1__SC_2_FORT_RUN: anon_1 = 50;
pub const anon_1__SC_2_SW_DEV: anon_1 = 51;
pub const anon_1__SC_2_LOCALEDEF: anon_1 = 52;
pub const anon_1__SC_PII: anon_1 = 53;
pub const anon_1__SC_PII_XTI: anon_1 = 54;
pub const anon_1__SC_PII_SOCKET: anon_1 = 55;
pub const anon_1__SC_PII_INTERNET: anon_1 = 56;
pub const anon_1__SC_PII_OSI: anon_1 = 57;
pub const anon_1__SC_POLL: anon_1 = 58;
pub const anon_1__SC_SELECT: anon_1 = 59;
pub const anon_1__SC_UIO_MAXIOV: anon_1 = 60;
pub const anon_1__SC_IOV_MAX: anon_1 = 60;
pub const anon_1__SC_PII_INTERNET_STREAM: anon_1 = 61;
pub const anon_1__SC_PII_INTERNET_DGRAM: anon_1 = 62;
pub const anon_1__SC_PII_OSI_COTS: anon_1 = 63;
pub const anon_1__SC_PII_OSI_CLTS: anon_1 = 64;
pub const anon_1__SC_PII_OSI_M: anon_1 = 65;
pub const anon_1__SC_T_IOV_MAX: anon_1 = 66;
pub const anon_1__SC_THREADS: anon_1 = 67;
pub const anon_1__SC_THREAD_SAFE_FUNCTIONS: anon_1 = 68;
pub const anon_1__SC_GETGR_R_SIZE_MAX: anon_1 = 69;
pub const anon_1__SC_GETPW_R_SIZE_MAX: anon_1 = 70;
pub const anon_1__SC_LOGIN_NAME_MAX: anon_1 = 71;
pub const anon_1__SC_TTY_NAME_MAX: anon_1 = 72;
pub const anon_1__SC_THREAD_DESTRUCTOR_ITERATIONS: anon_1 = 73;
pub const anon_1__SC_THREAD_KEYS_MAX: anon_1 = 74;
pub const anon_1__SC_THREAD_STACK_MIN: anon_1 = 75;
pub const anon_1__SC_THREAD_THREADS_MAX: anon_1 = 76;
pub const anon_1__SC_THREAD_ATTR_STACKADDR: anon_1 = 77;
pub const anon_1__SC_THREAD_ATTR_STACKSIZE: anon_1 = 78;
pub const anon_1__SC_THREAD_PRIORITY_SCHEDULING: anon_1 = 79;
pub const anon_1__SC_THREAD_PRIO_INHERIT: anon_1 = 80;
pub const anon_1__SC_THREAD_PRIO_PROTECT: anon_1 = 81;
pub const anon_1__SC_THREAD_PROCESS_SHARED: anon_1 = 82;
pub const anon_1__SC_NPROCESSORS_CONF: anon_1 = 83;
pub const anon_1__SC_NPROCESSORS_ONLN: anon_1 = 84;
pub const anon_1__SC_PHYS_PAGES: anon_1 = 85;
pub const anon_1__SC_AVPHYS_PAGES: anon_1 = 86;
pub const anon_1__SC_ATEXIT_MAX: anon_1 = 87;
pub const anon_1__SC_PASS_MAX: anon_1 = 88;
pub const anon_1__SC_XOPEN_VERSION: anon_1 = 89;
pub const anon_1__SC_XOPEN_XCU_VERSION: anon_1 = 90;
pub const anon_1__SC_XOPEN_UNIX: anon_1 = 91;
pub const anon_1__SC_XOPEN_CRYPT: anon_1 = 92;
pub const anon_1__SC_XOPEN_ENH_I18N: anon_1 = 93;
pub const anon_1__SC_XOPEN_SHM: anon_1 = 94;
pub const anon_1__SC_2_CHAR_TERM: anon_1 = 95;
pub const anon_1__SC_2_C_VERSION: anon_1 = 96;
pub const anon_1__SC_2_UPE: anon_1 = 97;
pub const anon_1__SC_XOPEN_XPG2: anon_1 = 98;
pub const anon_1__SC_XOPEN_XPG3: anon_1 = 99;
pub const anon_1__SC_XOPEN_XPG4: anon_1 = 100;
pub const anon_1__SC_CHAR_BIT: anon_1 = 101;
pub const anon_1__SC_CHAR_MAX: anon_1 = 102;
pub const anon_1__SC_CHAR_MIN: anon_1 = 103;
pub const anon_1__SC_INT_MAX: anon_1 = 104;
pub const anon_1__SC_INT_MIN: anon_1 = 105;
pub const anon_1__SC_LONG_BIT: anon_1 = 106;
pub const anon_1__SC_WORD_BIT: anon_1 = 107;
pub const anon_1__SC_MB_LEN_MAX: anon_1 = 108;
pub const anon_1__SC_NZERO: anon_1 = 109;
pub const anon_1__SC_SSIZE_MAX: anon_1 = 110;
pub const anon_1__SC_SCHAR_MAX: anon_1 = 111;
pub const anon_1__SC_SCHAR_MIN: anon_1 = 112;
pub const anon_1__SC_SHRT_MAX: anon_1 = 113;
pub const anon_1__SC_SHRT_MIN: anon_1 = 114;
pub const anon_1__SC_UCHAR_MAX: anon_1 = 115;
pub const anon_1__SC_UINT_MAX: anon_1 = 116;
pub const anon_1__SC_ULONG_MAX: anon_1 = 117;
pub const anon_1__SC_USHRT_MAX: anon_1 = 118;
pub const anon_1__SC_NL_ARGMAX: anon_1 = 119;
pub const anon_1__SC_NL_LANGMAX: anon_1 = 120;
pub const anon_1__SC_NL_MSGMAX: anon_1 = 121;
pub const anon_1__SC_NL_NMAX: anon_1 = 122;
pub const anon_1__SC_NL_SETMAX: anon_1 = 123;
pub const anon_1__SC_NL_TEXTMAX: anon_1 = 124;
pub const anon_1__SC_XBS5_ILP32_OFF32: anon_1 = 125;
pub const anon_1__SC_XBS5_ILP32_OFFBIG: anon_1 = 126;
pub const anon_1__SC_XBS5_LP64_OFF64: anon_1 = 127;
pub const anon_1__SC_XBS5_LPBIG_OFFBIG: anon_1 = 128;
pub const anon_1__SC_XOPEN_LEGACY: anon_1 = 129;
pub const anon_1__SC_XOPEN_REALTIME: anon_1 = 130;
pub const anon_1__SC_XOPEN_REALTIME_THREADS: anon_1 = 131;
pub const anon_1__SC_ADVISORY_INFO: anon_1 = 132;
pub const anon_1__SC_BARRIERS: anon_1 = 133;
pub const anon_1__SC_BASE: anon_1 = 134;
pub const anon_1__SC_C_LANG_SUPPORT: anon_1 = 135;
pub const anon_1__SC_C_LANG_SUPPORT_R: anon_1 = 136;
pub const anon_1__SC_CLOCK_SELECTION: anon_1 = 137;
pub const anon_1__SC_CPUTIME: anon_1 = 138;
pub const anon_1__SC_THREAD_CPUTIME: anon_1 = 139;
pub const anon_1__SC_DEVICE_IO: anon_1 = 140;
pub const anon_1__SC_DEVICE_SPECIFIC: anon_1 = 141;
pub const anon_1__SC_DEVICE_SPECIFIC_R: anon_1 = 142;
pub const anon_1__SC_FD_MGMT: anon_1 = 143;
pub const anon_1__SC_FIFO: anon_1 = 144;
pub const anon_1__SC_PIPE: anon_1 = 145;
pub const anon_1__SC_FILE_ATTRIBUTES: anon_1 = 146;
pub const anon_1__SC_FILE_LOCKING: anon_1 = 147;
pub const anon_1__SC_FILE_SYSTEM: anon_1 = 148;
pub const anon_1__SC_MONOTONIC_CLOCK: anon_1 = 149;
pub const anon_1__SC_MULTI_PROCESS: anon_1 = 150;
pub const anon_1__SC_SINGLE_PROCESS: anon_1 = 151;
pub const anon_1__SC_NETWORKING: anon_1 = 152;
pub const anon_1__SC_READER_WRITER_LOCKS: anon_1 = 153;
pub const anon_1__SC_SPIN_LOCKS: anon_1 = 154;
pub const anon_1__SC_REGEXP: anon_1 = 155;
pub const anon_1__SC_REGEX_VERSION: anon_1 = 156;
pub const anon_1__SC_SHELL: anon_1 = 157;
pub const anon_1__SC_SIGNALS: anon_1 = 158;
pub const anon_1__SC_SPAWN: anon_1 = 159;
pub const anon_1__SC_SPORADIC_SERVER: anon_1 = 160;
pub const anon_1__SC_THREAD_SPORADIC_SERVER: anon_1 = 161;
pub const anon_1__SC_SYSTEM_DATABASE: anon_1 = 162;
pub const anon_1__SC_SYSTEM_DATABASE_R: anon_1 = 163;
pub const anon_1__SC_TIMEOUTS: anon_1 = 164;
pub const anon_1__SC_TYPED_MEMORY_OBJECTS: anon_1 = 165;
pub const anon_1__SC_USER_GROUPS: anon_1 = 166;
pub const anon_1__SC_USER_GROUPS_R: anon_1 = 167;
pub const anon_1__SC_2_PBS: anon_1 = 168;
pub const anon_1__SC_2_PBS_ACCOUNTING: anon_1 = 169;
pub const anon_1__SC_2_PBS_LOCATE: anon_1 = 170;
pub const anon_1__SC_2_PBS_MESSAGE: anon_1 = 171;
pub const anon_1__SC_2_PBS_TRACK: anon_1 = 172;
pub const anon_1__SC_SYMLOOP_MAX: anon_1 = 173;
pub const anon_1__SC_STREAMS: anon_1 = 174;
pub const anon_1__SC_2_PBS_CHECKPOINT: anon_1 = 175;
pub const anon_1__SC_V6_ILP32_OFF32: anon_1 = 176;
pub const anon_1__SC_V6_ILP32_OFFBIG: anon_1 = 177;
pub const anon_1__SC_V6_LP64_OFF64: anon_1 = 178;
pub const anon_1__SC_V6_LPBIG_OFFBIG: anon_1 = 179;
pub const anon_1__SC_HOST_NAME_MAX: anon_1 = 180;
pub const anon_1__SC_TRACE: anon_1 = 181;
pub const anon_1__SC_TRACE_EVENT_FILTER: anon_1 = 182;
pub const anon_1__SC_TRACE_INHERIT: anon_1 = 183;
pub const anon_1__SC_TRACE_LOG: anon_1 = 184;
pub const anon_1__SC_LEVEL1_ICACHE_SIZE: anon_1 = 185;
pub const anon_1__SC_LEVEL1_ICACHE_ASSOC: anon_1 = 186;
pub const anon_1__SC_LEVEL1_ICACHE_LINESIZE: anon_1 = 187;
pub const anon_1__SC_LEVEL1_DCACHE_SIZE: anon_1 = 188;
pub const anon_1__SC_LEVEL1_DCACHE_ASSOC: anon_1 = 189;
pub const anon_1__SC_LEVEL1_DCACHE_LINESIZE: anon_1 = 190;
pub const anon_1__SC_LEVEL2_CACHE_SIZE: anon_1 = 191;
pub const anon_1__SC_LEVEL2_CACHE_ASSOC: anon_1 = 192;
pub const anon_1__SC_LEVEL2_CACHE_LINESIZE: anon_1 = 193;
pub const anon_1__SC_LEVEL3_CACHE_SIZE: anon_1 = 194;
pub const anon_1__SC_LEVEL3_CACHE_ASSOC: anon_1 = 195;
pub const anon_1__SC_LEVEL3_CACHE_LINESIZE: anon_1 = 196;
pub const anon_1__SC_LEVEL4_CACHE_SIZE: anon_1 = 197;
pub const anon_1__SC_LEVEL4_CACHE_ASSOC: anon_1 = 198;
pub const anon_1__SC_LEVEL4_CACHE_LINESIZE: anon_1 = 199;
pub const anon_1__SC_IPV6: anon_1 = 235;
pub const anon_1__SC_RAW_SOCKETS: anon_1 = 236;
pub const anon_1__SC_V7_ILP32_OFF32: anon_1 = 237;
pub const anon_1__SC_V7_ILP32_OFFBIG: anon_1 = 238;
pub const anon_1__SC_V7_LP64_OFF64: anon_1 = 239;
pub const anon_1__SC_V7_LPBIG_OFFBIG: anon_1 = 240;
pub const anon_1__SC_SS_REPL_MAX: anon_1 = 241;
pub const anon_1__SC_TRACE_EVENT_NAME_MAX: anon_1 = 242;
pub const anon_1__SC_TRACE_NAME_MAX: anon_1 = 243;
pub const anon_1__SC_TRACE_SYS_MAX: anon_1 = 244;
pub const anon_1__SC_TRACE_USER_EVENT_MAX: anon_1 = 245;
pub const anon_1__SC_XOPEN_STREAMS: anon_1 = 246;
pub const anon_1__SC_THREAD_ROBUST_PRIO_INHERIT: anon_1 = 247;
pub const anon_1__SC_THREAD_ROBUST_PRIO_PROTECT: anon_1 = 248;
pub const anon_1__SC_MINSIGSTKSZ: anon_1 = 249;
pub const anon_1__SC_SIGSTKSZ: anon_1 = 250;
pub unsafe fn test_fputc_fputs_2() {
    let mut path: *const libc::c_char =
        (c"cpp2rust_stdio_nofd_puts.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut _IO_FILE =
        (unsafe { fopen_3(path, (c"wb".as_ptr().cast_mut()).cast_const()) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((fp).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fp != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        9_u32,
                        c"void test_fputc_fputs(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fputc_5(('A' as i32), fp) }) == ('A' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fputc(\'A\', fp) == \'A\'".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        10_u32,
                        c"void test_fputc_fputs(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fputs_6((c"BCD\n".as_ptr().cast_mut()).cast_const(), fp) }) >= (0))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fputs(\"BCD\\n\", fp) >= 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        11_u32,
                        c"void test_fputc_fputs(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fclose_7(fp) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fclose(fp) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        12_u32,
                        c"void test_fputc_fputs(void)".as_ptr(),
                    )
                });
            }
        }
    };
    fp = (unsafe { fopen_3(path, (c"rb".as_ptr().cast_mut()).cast_const()) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((fp).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fp != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        14_u32,
                        c"void test_fputc_fputs(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut buf: [libc::c_char; 16] = [
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
    ];
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                fread_8(
                    (buf.as_mut_ptr() as *mut ::libc::c_void),
                    1_usize,
                    16_usize,
                    fp,
                )
            }) == (5_usize)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fread(buf, 1, 16, fp) == 5".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        16_u32,
                        c"void test_fputc_fputs(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                memcmp_9(
                    (buf.as_mut_ptr() as *const ::libc::c_void),
                    (c"ABCD\n".as_ptr().cast_mut() as *const ::libc::c_void),
                    5_usize,
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"memcmp(buf, \"ABCD\\n\", 5) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        17_u32,
                        c"void test_fputc_fputs(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fclose_7(fp) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fclose(fp) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        18_u32,
                        c"void test_fputc_fputs(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { unlink_10(path) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"unlink(path) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        19_u32,
                        c"void test_fputc_fputs(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_puts_11() {
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { puts_12((c"hello from puts".as_ptr().cast_mut()).cast_const()) })
                >= (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"puts(\"hello from puts\") >= 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        22_u32,
                        c"void test_puts(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_fgets_getc_13() {
    let mut path: *const libc::c_char =
        (c"cpp2rust_stdio_nofd_gets.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut _IO_FILE =
        (unsafe { fopen_3(path, (c"wb".as_ptr().cast_mut()).cast_const()) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((fp).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fp != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        27_u32,
                        c"void test_fgets_getc(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fputs_6((c"line1\nline2\n".as_ptr().cast_mut()).cast_const(), fp) })
                >= (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fputs(\"line1\\nline2\\n\", fp) >= 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        28_u32,
                        c"void test_fgets_getc(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fclose_7(fp) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fclose(fp) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        29_u32,
                        c"void test_fgets_getc(void)".as_ptr(),
                    )
                });
            }
        }
    };
    fp = (unsafe { fopen_3(path, (c"rb".as_ptr().cast_mut()).cast_const()) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((fp).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fp != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        31_u32,
                        c"void test_fgets_getc(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut buf: [libc::c_char; 8] = [(0 as libc::c_char); 8];
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((unsafe { fgets_14(buf.as_mut_ptr(), 8, fp) }).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fgets(buf, 8, fp) != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        33_u32,
                        c"void test_fgets_getc(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                memcmp_9(
                    (buf.as_mut_ptr() as *const ::libc::c_void),
                    (c"line1\n".as_ptr().cast_mut() as *const ::libc::c_void),
                    7_usize,
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"memcmp(buf, \"line1\\n\", 7) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        34_u32,
                        c"void test_fgets_getc(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { getc_15(fp) }) == ('l' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"getc(fp) == \'l\'".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        35_u32,
                        c"void test_fgets_getc(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((unsafe { fgets_14(buf.as_mut_ptr(), 4, fp) }).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fgets(buf, 4, fp) != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        36_u32,
                        c"void test_fgets_getc(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                memcmp_9(
                    (buf.as_mut_ptr() as *const ::libc::c_void),
                    (c"ine".as_ptr().cast_mut() as *const ::libc::c_void),
                    4_usize,
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"memcmp(buf, \"ine\", 4) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        37_u32,
                        c"void test_fgets_getc(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((unsafe { fgets_14(buf.as_mut_ptr(), 8, fp) }).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fgets(buf, 8, fp) != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        38_u32,
                        c"void test_fgets_getc(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                memcmp_9(
                    (buf.as_mut_ptr() as *const ::libc::c_void),
                    (c"2\n".as_ptr().cast_mut() as *const ::libc::c_void),
                    3_usize,
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"memcmp(buf, \"2\\n\", 3) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        39_u32,
                        c"void test_fgets_getc(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fgets_14(buf.as_mut_ptr(), 8, fp) }).is_null()) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fgets(buf, 8, fp) == NULL".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        40_u32,
                        c"void test_fgets_getc(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { getc_15(fp) }) == (-1_i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"getc(fp) == EOF".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        41_u32,
                        c"void test_fgets_getc(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fclose_7(fp) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fclose(fp) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        42_u32,
                        c"void test_fgets_getc(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { unlink_10(path) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"unlink(path) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        43_u32,
                        c"void test_fgets_getc(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_freopen_16() {
    let mut path: *const libc::c_char =
        (c"cpp2rust_stdio_nofd_reopen.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut _IO_FILE =
        (unsafe { fopen_3(path, (c"wb".as_ptr().cast_mut()).cast_const()) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((fp).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fp != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        49_u32,
                        c"void test_freopen(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fputs_6((c"hello".as_ptr().cast_mut()).cast_const(), fp) }) >= (0))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fputs(\"hello\", fp) >= 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        50_u32,
                        c"void test_freopen(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut fp2: *mut _IO_FILE =
        (unsafe { freopen_17(path, (c"rb".as_ptr().cast_mut()).cast_const(), fp) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((fp2).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fp2 != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        52_u32,
                        c"void test_freopen(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut buf: [libc::c_char; 8] = [
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
    ];
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                fread_8(
                    (buf.as_mut_ptr() as *mut ::libc::c_void),
                    1_usize,
                    8_usize,
                    fp2,
                )
            }) == (5_usize)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fread(buf, 1, 8, fp2) == 5".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        54_u32,
                        c"void test_freopen(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                memcmp_9(
                    (buf.as_mut_ptr() as *const ::libc::c_void),
                    (c"hello".as_ptr().cast_mut() as *const ::libc::c_void),
                    5_usize,
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"memcmp(buf, \"hello\", 5) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        55_u32,
                        c"void test_freopen(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fclose_7(fp2) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fclose(fp2) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        56_u32,
                        c"void test_freopen(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { unlink_10(path) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"unlink(path) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        57_u32,
                        c"void test_freopen(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_fseeko_18() {
    let mut path: *const libc::c_char =
        (c"cpp2rust_stdio_nofd_seek.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut _IO_FILE =
        (unsafe { fopen_3(path, (c"wb".as_ptr().cast_mut()).cast_const()) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((fp).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fp != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        63_u32,
                        c"void test_fseeko(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fputs_6((c"hello world".as_ptr().cast_mut()).cast_const(), fp) })
                >= (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fputs(\"hello world\", fp) >= 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        64_u32,
                        c"void test_fseeko(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fclose_7(fp) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fclose(fp) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        65_u32,
                        c"void test_fseeko(void)".as_ptr(),
                    )
                });
            }
        }
    };
    fp = (unsafe { fopen_3(path, (c"rb".as_ptr().cast_mut()).cast_const()) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((fp).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fp != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        67_u32,
                        c"void test_fseeko(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fseeko_19(fp, 6_i64, 0) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fseeko(fp, 6, SEEK_SET) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        68_u32,
                        c"void test_fseeko(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut buf: [libc::c_char; 8] = [
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
    ];
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                fread_8(
                    (buf.as_mut_ptr() as *mut ::libc::c_void),
                    1_usize,
                    5_usize,
                    fp,
                )
            }) == (5_usize)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fread(buf, 1, 5, fp) == 5".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        70_u32,
                        c"void test_fseeko(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                memcmp_9(
                    (buf.as_mut_ptr() as *const ::libc::c_void),
                    (c"world".as_ptr().cast_mut() as *const ::libc::c_void),
                    5_usize,
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"memcmp(buf, \"world\", 5) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        71_u32,
                        c"void test_fseeko(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fseeko_19(fp, (-5_i32 as i64), 2) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fseeko(fp, -5, SEEK_END) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        72_u32,
                        c"void test_fseeko(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { getc_15(fp) }) == ('w' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"getc(fp) == \'w\'".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        73_u32,
                        c"void test_fseeko(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fseeko_19(fp, 1_i64, 1) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fseeko(fp, 1, SEEK_CUR) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        74_u32,
                        c"void test_fseeko(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { getc_15(fp) }) == ('r' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"getc(fp) == \'r\'".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        75_u32,
                        c"void test_fseeko(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fclose_7(fp) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fclose(fp) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        76_u32,
                        c"void test_fseeko(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { unlink_10(path) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"unlink(path) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        77_u32,
                        c"void test_fseeko(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_rename_20() {
    let mut from: *const libc::c_char =
        (c"cpp2rust_stdio_nofd_from.tmp".as_ptr().cast_mut()).cast_const();
    let mut to: *const libc::c_char =
        (c"cpp2rust_stdio_nofd_to.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut _IO_FILE =
        (unsafe { fopen_3(from, (c"wb".as_ptr().cast_mut()).cast_const()) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((fp).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fp != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        84_u32,
                        c"void test_rename(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fputs_6((c"data".as_ptr().cast_mut()).cast_const(), fp) }) >= (0))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fputs(\"data\", fp) >= 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        85_u32,
                        c"void test_rename(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fclose_7(fp) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fclose(fp) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        86_u32,
                        c"void test_rename(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { rename_21(from, to) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"rename(from, to) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        87_u32,
                        c"void test_rename(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fopen_3(from, (c"rb".as_ptr().cast_mut()).cast_const()) }).is_null())
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fopen(from, \"rb\") == NULL".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        88_u32,
                        c"void test_rename(void)".as_ptr(),
                    )
                });
            }
        }
    };
    fp = (unsafe { fopen_3(to, (c"rb".as_ptr().cast_mut()).cast_const()) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((fp).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fp != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        90_u32,
                        c"void test_rename(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fclose_7(fp) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fclose(fp) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        91_u32,
                        c"void test_rename(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { rename_21(from, to) }) == (-1_i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"rename(from, to) == -1".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        92_u32,
                        c"void test_rename(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { unlink_10(to) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"unlink(to) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        93_u32,
                        c"void test_rename(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_setvbuf_22() {
    let mut path: *const libc::c_char =
        (c"cpp2rust_stdio_nofd_vbuf.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut _IO_FILE =
        (unsafe { fopen_3(path, (c"wb".as_ptr().cast_mut()).cast_const()) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((fp).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fp != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        99_u32,
                        c"void test_setvbuf(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { setvbuf_23(fp, std::ptr::null_mut(), 2, 0_usize) }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"setvbuf(fp, NULL, _IONBF, 0) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        100_u32,
                        c"void test_setvbuf(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fputs_6((c"x".as_ptr().cast_mut()).cast_const(), fp) }) >= (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fputs(\"x\", fp) >= 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        101_u32,
                        c"void test_setvbuf(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { fclose_7(fp) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"fclose(fp) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        102_u32,
                        c"void test_setvbuf(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { unlink_10(path) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"unlink(path) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"stdio_nofd.c".as_ptr().cast_mut()).cast_const(),
                        103_u32,
                        c"void test_setvbuf(void)".as_ptr(),
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
    (unsafe { test_fputc_fputs_2() });
    (unsafe { test_puts_11() });
    (unsafe { test_fgets_getc_13() });
    (unsafe { test_freopen_16() });
    (unsafe { test_fseeko_18() });
    (unsafe { test_rename_20() });
    (unsafe { test_setvbuf_22() });
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_wide_data;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_codecvt;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_marker;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
pub unsafe fn fopen_3(
    mut __filename: *const libc::c_char,
    mut __modes: *const libc::c_char,
) -> *mut _IO_FILE {
    unimplemented!()
}
pub unsafe fn __assert_fail_4(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn fputc_5(mut __c: i32, mut __stream: *mut _IO_FILE) -> i32 {
    unimplemented!()
}
pub unsafe fn fputs_6(mut __s: *const libc::c_char, mut __stream: *mut _IO_FILE) -> i32 {
    unimplemented!()
}
pub unsafe fn fclose_7(mut __stream: *mut _IO_FILE) -> i32 {
    unimplemented!()
}
pub unsafe fn fread_8(
    mut __ptr: *mut ::libc::c_void,
    mut __size: usize,
    mut __n: usize,
    mut __stream: *mut _IO_FILE,
) -> usize {
    unimplemented!()
}
pub unsafe fn memcmp_9(
    mut __s1: *const ::libc::c_void,
    mut __s2: *const ::libc::c_void,
    mut __n: usize,
) -> i32 {
    unimplemented!()
}
pub unsafe fn unlink_10(mut __name: *const libc::c_char) -> i32 {
    unimplemented!()
}
pub unsafe fn puts_12(mut __s: *const libc::c_char) -> i32 {
    unimplemented!()
}
pub unsafe fn fgets_14(
    mut __s: *mut libc::c_char,
    mut __n: i32,
    mut __stream: *mut _IO_FILE,
) -> *mut libc::c_char {
    unimplemented!()
}
pub unsafe fn getc_15(mut __stream: *mut _IO_FILE) -> i32 {
    unimplemented!()
}
pub unsafe fn freopen_17(
    mut __filename: *const libc::c_char,
    mut __modes: *const libc::c_char,
    mut __stream: *mut _IO_FILE,
) -> *mut _IO_FILE {
    unimplemented!()
}
pub unsafe fn fseeko_19(mut __stream: *mut _IO_FILE, mut __off: i64, mut __whence: i32) -> i32 {
    unimplemented!()
}
pub unsafe fn rename_21(mut __old: *const libc::c_char, mut __new: *const libc::c_char) -> i32 {
    unimplemented!()
}
pub unsafe fn setvbuf_23(
    mut __stream: *mut _IO_FILE,
    mut __buf: *mut libc::c_char,
    mut __modes: i32,
    mut __n: usize,
) -> i32 {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
