extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __fsid_t {
    #[offset(0)]
    #[byte_size(8)]
    pub __val: Value<Box<[i32]>>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __mbstate_t {
    #[offset(0)]
    pub __count: i32,
    #[offset(4)]
    #[byte_size(4)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct anon_0 {
    #[offset(0)]
    pub __wch: u32,
    #[offset(0)]
    #[byte_size(4)]
    pub __wchb: Value<Box<[i8]>>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _G_fpos_t {
    #[offset(0)]
    pub __pos: i64,
    #[offset(8)]
    #[byte_size(8)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _G_fpos64_t {
    #[offset(0)]
    pub __pos: i64,
    #[offset(8)]
    #[byte_size(8)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(216)]
pub struct _IO_FILE {
    #[offset(0)]
    pub _flags: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub _IO_read_ptr: Ptr<i8>,
    #[offset(16)]
    #[byte_size(8)]
    pub _IO_read_end: Ptr<i8>,
    #[offset(24)]
    #[byte_size(8)]
    pub _IO_read_base: Ptr<i8>,
    #[offset(32)]
    #[byte_size(8)]
    pub _IO_write_base: Ptr<i8>,
    #[offset(40)]
    #[byte_size(8)]
    pub _IO_write_ptr: Ptr<i8>,
    #[offset(48)]
    #[byte_size(8)]
    pub _IO_write_end: Ptr<i8>,
    #[offset(56)]
    #[byte_size(8)]
    pub _IO_buf_base: Ptr<i8>,
    #[offset(64)]
    #[byte_size(8)]
    pub _IO_buf_end: Ptr<i8>,
    #[offset(72)]
    #[byte_size(8)]
    pub _IO_save_base: Ptr<i8>,
    #[offset(80)]
    #[byte_size(8)]
    pub _IO_backup_base: Ptr<i8>,
    #[offset(88)]
    #[byte_size(8)]
    pub _IO_save_end: Ptr<i8>,
    #[offset(96)]
    #[byte_size(8)]
    pub _markers: Ptr<_IO_marker>,
    #[offset(104)]
    #[byte_size(8)]
    pub _chain: Ptr<_IO_FILE>,
    #[offset(112)]
    pub _fileno: i32,
    #[offset(116)]
    pub _flags2: i32,
    #[offset(120)]
    pub _old_offset: i64,
    #[offset(128)]
    pub _cur_column: u16,
    #[offset(130)]
    pub _vtable_offset: i8,
    #[offset(131)]
    #[byte_size(1)]
    pub _shortbuf: Value<Box<[i8]>>,
    #[offset(136)]
    #[byte_size(8)]
    pub _lock: AnyPtr,
    #[offset(144)]
    pub _offset: i64,
    #[offset(152)]
    #[byte_size(8)]
    pub _codecvt: Ptr<_IO_codecvt>,
    #[offset(160)]
    #[byte_size(8)]
    pub _wide_data: Ptr<_IO_wide_data>,
    #[offset(168)]
    #[byte_size(8)]
    pub _freeres_list: Ptr<_IO_FILE>,
    #[offset(176)]
    #[byte_size(8)]
    pub _freeres_buf: AnyPtr,
    #[offset(184)]
    pub __pad5: usize,
    #[offset(192)]
    pub _mode: i32,
    #[offset(196)]
    #[byte_size(20)]
    pub _unused2: Value<Box<[i8]>>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct _IO_cookie_io_functions_t {
    #[offset(0)]
    #[byte_size(8)]
    pub read: FnPtr<fn(AnyPtr, Ptr<i8>, usize) -> i64>,
    #[offset(8)]
    #[byte_size(8)]
    pub write: FnPtr<fn(AnyPtr, Ptr<i8>, usize) -> i64>,
    #[offset(16)]
    #[byte_size(8)]
    pub seek: FnPtr<fn(AnyPtr, Ptr<i64>, i32) -> i32>,
    #[offset(24)]
    #[byte_size(8)]
    pub close: FnPtr<fn(AnyPtr) -> i32>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct timeval {
    #[offset(0)]
    pub tv_sec: i64,
    #[offset(8)]
    pub tv_usec: i64,
}
impl Clone for timeval {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timeval {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct __sigset_t {
    #[offset(0)]
    #[byte_size(128)]
    pub __val: Value<Box<[u64]>>,
}
impl Clone for __sigset_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __sigset_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct timespec {
    #[offset(0)]
    pub tv_sec: i64,
    #[offset(8)]
    pub tv_nsec: i64,
}
impl Clone for timespec {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timespec {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct fd_set {
    #[offset(0)]
    #[byte_size(128)]
    pub __fds_bits: Value<Box<[i64]>>,
}
impl Clone for fd_set {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for fd_set {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct timezone {
    #[offset(0)]
    pub tz_minuteswest: i32,
    #[offset(4)]
    pub tz_dsttime: i32,
}
impl Clone for timezone {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timezone {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct itimerval {
    #[offset(0)]
    #[byte_size(16)]
    pub it_interval: timeval,
    #[offset(16)]
    #[byte_size(16)]
    pub it_value: timeval,
}
impl Clone for itimerval {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for itimerval {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct tm {
    #[offset(0)]
    pub tm_sec: i32,
    #[offset(4)]
    pub tm_min: i32,
    #[offset(8)]
    pub tm_hour: i32,
    #[offset(12)]
    pub tm_mday: i32,
    #[offset(16)]
    pub tm_mon: i32,
    #[offset(20)]
    pub tm_year: i32,
    #[offset(24)]
    pub tm_wday: i32,
    #[offset(28)]
    pub tm_yday: i32,
    #[offset(32)]
    pub tm_isdst: i32,
    #[offset(40)]
    pub tm_gmtoff: i64,
    #[offset(48)]
    #[byte_size(8)]
    pub tm_zone: Ptr<i8>,
}
impl Clone for tm {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tm {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct itimerspec {
    #[offset(0)]
    #[byte_size(16)]
    pub it_interval: timespec,
    #[offset(16)]
    #[byte_size(16)]
    pub it_value: timespec,
}
impl Clone for itimerspec {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for itimerspec {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(232)]
pub struct __locale_struct {
    #[offset(0)]
    #[byte_size(104)]
    pub __locales: Value<Box<[Ptr<__locale_data>]>>,
    #[offset(104)]
    #[byte_size(8)]
    pub __ctype_b: Ptr<u16>,
    #[offset(112)]
    #[byte_size(8)]
    pub __ctype_tolower: Ptr<i32>,
    #[offset(120)]
    #[byte_size(8)]
    pub __ctype_toupper: Ptr<i32>,
    #[offset(128)]
    #[byte_size(104)]
    pub __names: Value<Box<[Ptr<i8>]>>,
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
pub fn test_time_2() {
    let mut t1: i64 = ({ time_3(Ptr::<i64>::null()) });
    let t2: Value<i64> = Rc::new(RefCell::new(0_i64));
    let mut t3: i64 = ({ time_3((t2.as_pointer())) });
    {
        &(4usize);
        {
            if (((t1 > 1500000000_i64) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"t1 > 1500000000"),
                        Ptr::<i8>::from_string_literal(b"sys_time.c"),
                        11_u32,
                        Ptr::<i8>::from_string_literal(b"void test_time(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((*t2.borrow()) == t3) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"t2 == t3"),
                        Ptr::<i8>::from_string_literal(b"sys_time.c"),
                        12_u32,
                        Ptr::<i8>::from_string_literal(b"void test_time(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((t3 >= t1) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"t3 >= t1"),
                        Ptr::<i8>::from_string_literal(b"sys_time.c"),
                        13_u32,
                        Ptr::<i8>::from_string_literal(b"void test_time(void)"),
                    )
                });
            }
        }
    };
}
pub fn print_tm_5(t: i64) {
    let t: Value<i64> = Rc::new(RefCell::new(t));
    let tm: Value<tm> = <Value<tm>>::default();
    {
        &(4usize);
        {
            if (((!(({ gmtime_r_6((t.as_pointer()), (tm.as_pointer())) }).is_null())) as i32) != 0)
            {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"gmtime_r(&t, &tm) != NULL"),
                        Ptr::<i8>::from_string_literal(b"sys_time.c"),
                        18_u32,
                        Ptr::<i8>::from_string_literal(b"void print_tm(time_t)"),
                    )
                });
            }
        }
    };
    println!(
        "{}-{}-{} {}:{}:{} wday={} yday={} {} gmtoff={} isdst={}",
        { (*tm.borrow()).tm_year },
        { (*tm.borrow()).tm_mon },
        { (*tm.borrow()).tm_mday },
        { (*tm.borrow()).tm_hour },
        { (*tm.borrow()).tm_min },
        { (*tm.borrow()).tm_sec },
        { (*tm.borrow()).tm_wday },
        { (*tm.borrow()).tm_yday },
        { (*tm.borrow()).tm_zone.clone() },
        { (*tm.borrow()).tm_gmtoff },
        { (*tm.borrow()).tm_isdst }
    );
}
pub fn test_gmtime_r_7() {
    ({ print_tm_5(0_i64) });
    ({ print_tm_5(1_i64) });
    ({ print_tm_5(86399_i64) });
    ({ print_tm_5(86400_i64) });
    ({ print_tm_5(951782400_i64) });
    ({ print_tm_5(951868799_i64) });
    ({ print_tm_5(1704067199_i64) });
    ({ print_tm_5(1704067200_i64) });
    ({ print_tm_5(1721126096_i64) });
    ({ print_tm_5(4102444800_i64) });
}
pub fn print_local_tm_8(t: i64) {
    let t: Value<i64> = Rc::new(RefCell::new(t));
    let tm: Value<tm> = <Value<tm>>::default();
    {
        &(4usize);
        {
            if (((!(({ localtime_r_9((t.as_pointer()), (tm.as_pointer())) }).is_null())) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"localtime_r(&t, &tm) != NULL"),
                        Ptr::<i8>::from_string_literal(b"sys_time.c"),
                        39_u32,
                        Ptr::<i8>::from_string_literal(b"void print_local_tm(time_t)"),
                    )
                });
            }
        }
    };
    println!(
        "{}-{}-{} {}:{}:{} wday={} yday={} {} gmtoff={} isdst={}",
        { (*tm.borrow()).tm_year },
        { (*tm.borrow()).tm_mon },
        { (*tm.borrow()).tm_mday },
        { (*tm.borrow()).tm_hour },
        { (*tm.borrow()).tm_min },
        { (*tm.borrow()).tm_sec },
        { (*tm.borrow()).tm_wday },
        { (*tm.borrow()).tm_yday },
        { (*tm.borrow()).tm_zone.clone() },
        { (*tm.borrow()).tm_gmtoff },
        { (*tm.borrow()).tm_isdst }
    );
}
pub fn test_localtime_r_10() {
    ({ print_local_tm_8(0_i64) });
    ({ print_local_tm_8(951782400_i64) });
    ({ print_local_tm_8(1704067199_i64) });
    ({ print_local_tm_8(1721126096_i64) });
    ({ print_local_tm_8(1735689600_i64) });
}
pub fn test_strftime_11() {
    let t: Value<i64> = Rc::new(RefCell::new(1721126096_i64));
    let tm: Value<tm> = <Value<tm>>::default();
    {
        &(4usize);
        {
            if (((!(({ gmtime_r_6((t.as_pointer()), (tm.as_pointer())) }).is_null())) as i32) != 0)
            {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"gmtime_r(&t, &tm) != NULL"),
                        Ptr::<i8>::from_string_literal(b"sys_time.c"),
                        56_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strftime(void)"),
                    )
                });
            }
        }
    };
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new((0..64).map(|_| 0_i8).collect::<Box<[i8]>>()));
    {
        &(4usize);
        {
            if (((({
                let ___s: Ptr<i8> = (buf.as_pointer() as Ptr<i8>);
                let ___maxsize: usize = 64usize;
                strftime_12(
                    ___s,
                    ___maxsize,
                    Ptr::<i8>::from_string_literal(b"%Y-%m-%d %H:%M:%S"),
                    (tm.as_pointer()),
                )
            }) > 0_usize) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(
                            b"strftime(buf, sizeof(buf), \"%Y-%m-%d %H:%M:%S\", &tm) > 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"sys_time.c"),
                        58_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strftime(void)"),
                    )
                });
            }
        }
    };
    println!("{}", (buf.as_pointer() as Ptr::<i8>));
    {
        &(4usize);
        {
            if (((({
                let ___s: Ptr<i8> = (buf.as_pointer() as Ptr<i8>);
                let ___maxsize: usize = 64usize;
                strftime_12(
                    ___s,
                    ___maxsize,
                    Ptr::<i8>::from_string_literal(b"%a, %d %b %Y %T"),
                    (tm.as_pointer()),
                )
            }) > 0_usize) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(
                            b"strftime(buf, sizeof(buf), \"%a, %d %b %Y %T\", &tm) > 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"sys_time.c"),
                        60_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strftime(void)"),
                    )
                });
            }
        }
    };
    println!("{}", (buf.as_pointer() as Ptr::<i8>));
    {
        &(4usize);
        {
            if (((({
                let ___s: Ptr<i8> = (buf.as_pointer() as Ptr<i8>);
                let ___maxsize: usize = 64usize;
                strftime_12(
                    ___s,
                    ___maxsize,
                    Ptr::<i8>::from_string_literal(b"day %j 100%%"),
                    (tm.as_pointer()),
                )
            }) > 0_usize) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(
                            b"strftime(buf, sizeof(buf), \"day %j 100%%\", &tm) > 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"sys_time.c"),
                        62_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strftime(void)"),
                    )
                });
            }
        }
    };
    println!("{}", (buf.as_pointer() as Ptr::<i8>));
    {
        &(4usize);
        {
            if (((({
                let ___s: Ptr<i8> = (buf.as_pointer() as Ptr<i8>);
                let ___maxsize: usize = 64usize;
                strftime_12(
                    ___s,
                    ___maxsize,
                    Ptr::<i8>::from_string_literal(b"%e"),
                    (tm.as_pointer()),
                )
            }) > 0_usize) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(
                            b"strftime(buf, sizeof(buf), \"%e\", &tm) > 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"sys_time.c"),
                        64_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strftime(void)"),
                    )
                });
            }
        }
    };
    println!("{}", (buf.as_pointer() as Ptr::<i8>));
    let small: Value<Box<[i8]>> =
        Rc::new(RefCell::new((0..4).map(|_| 0_i8).collect::<Box<[i8]>>()));
    {
        &(4usize);
        {
            if (((({
                let ___s: Ptr<i8> = (small.as_pointer() as Ptr<i8>);
                let ___maxsize: usize = 4usize;
                strftime_12(
                    ___s,
                    ___maxsize,
                    Ptr::<i8>::from_string_literal(b"%Y-%m-%d"),
                    (tm.as_pointer()),
                )
            }) == 0_usize) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(
                            b"strftime(small, sizeof(small), \"%Y-%m-%d\", &tm) == 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"sys_time.c"),
                        67_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strftime(void)"),
                    )
                });
            }
        }
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_time_2() });
    ({ test_gmtime_r_7() });
    ({ test_localtime_r_10() });
    ({ test_strftime_11() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __locale_data;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_marker;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_codecvt;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_wide_data;
pub fn time_3(mut __timer: Ptr<i64>) -> i64 {
    unimplemented!()
}
pub fn __assert_fail_4(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn gmtime_r_6(mut __timer: Ptr<i64>, mut __tp: Ptr<tm>) -> Ptr<tm> {
    unimplemented!()
}
pub fn localtime_r_9(mut __timer: Ptr<i64>, mut __tp: Ptr<tm>) -> Ptr<tm> {
    unimplemented!()
}
pub fn strftime_12(
    mut __s: Ptr<i8>,
    mut __maxsize: usize,
    mut __format: Ptr<i8>,
    mut __tp: Ptr<tm>,
) -> usize {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
