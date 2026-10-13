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
#[byte_size(144)]
pub struct stat {
    #[offset(0)]
    pub st_dev: u64,
    #[offset(8)]
    pub st_ino: u64,
    #[offset(16)]
    pub st_nlink: u64,
    #[offset(24)]
    pub st_mode: u32,
    #[offset(28)]
    pub st_uid: u32,
    #[offset(32)]
    pub st_gid: u32,
    #[offset(36)]
    pub __pad0: i32,
    #[offset(40)]
    pub st_rdev: u64,
    #[offset(48)]
    pub st_size: i64,
    #[offset(56)]
    pub st_blksize: i64,
    #[offset(64)]
    pub st_blocks: i64,
    #[offset(72)]
    #[byte_size(16)]
    pub st_atim: timespec,
    #[offset(88)]
    #[byte_size(16)]
    pub st_mtim: timespec,
    #[offset(104)]
    #[byte_size(16)]
    pub st_ctim: timespec,
    #[offset(120)]
    #[byte_size(24)]
    pub __glibc_reserved: Value<Box<[i64]>>,
}
impl Clone for stat {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for stat {
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
pub fn test_stat_2() {
    let mut path: Ptr<i8> = Ptr::<i8>::from_string_literal(b"cpp2rust_stat_test.tmp");
    let mut fp: Ptr<_IO_FILE> =
        ({ fopen_3((path).clone(), Ptr::<i8>::from_string_literal(b"wb")) });
    {
        &(4usize);
        {
            if (((!((fp).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"fp != NULL"),
                        Ptr::<i8>::from_string_literal(b"sys_stat.c"),
                        10_u32,
                        Ptr::<i8>::from_string_literal(b"void test_stat(void)"),
                    )
                });
            }
        }
    };
    ({ fputs_5(Ptr::<i8>::from_string_literal(b"hello"), (fp).clone()) });
    {
        &(4usize);
        {
            if (((({ fclose_6((fp).clone()) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"fclose(fp) == 0"),
                        Ptr::<i8>::from_string_literal(b"sys_stat.c"),
                        12_u32,
                        Ptr::<i8>::from_string_literal(b"void test_stat(void)"),
                    )
                });
            }
        }
    };
    let st: Value<stat> = <Value<stat>>::default();
    {
        &(4usize);
        {
            if (((({ stat_7((path).clone(), (st.as_pointer())) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"stat(path, &st) == 0"),
                        Ptr::<i8>::from_string_literal(b"sys_stat.c"),
                        14_u32,
                        Ptr::<i8>::from_string_literal(b"void test_stat(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((({ (*st.borrow()).st_size } == 5_i64) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"st.st_size == 5"),
                        Ptr::<i8>::from_string_literal(b"sys_stat.c"),
                        15_u32,
                        Ptr::<i8>::from_string_literal(b"void test_stat(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((({ (*st.borrow()).st_mtime } > 0_i64) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"st.st_mtime > 0"),
                        Ptr::<i8>::from_string_literal(b"sys_stat.c"),
                        16_u32,
                        Ptr::<i8>::from_string_literal(b"void test_stat(void)"),
                    )
                });
            }
        }
    };
    ({ unlink_8((path).clone()) });
}
pub fn test_fstat_9() {
    let mut path: Ptr<i8> = Ptr::<i8>::from_string_literal(b"cpp2rust_fstat_test.tmp");
    let mut fp: Ptr<_IO_FILE> =
        ({ fopen_3((path).clone(), Ptr::<i8>::from_string_literal(b"wb")) });
    {
        &(4usize);
        {
            if (((!((fp).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"fp != NULL"),
                        Ptr::<i8>::from_string_literal(b"sys_stat.c"),
                        23_u32,
                        Ptr::<i8>::from_string_literal(b"void test_fstat(void)"),
                    )
                });
            }
        }
    };
    ({ fputs_5(Ptr::<i8>::from_string_literal(b"hello world"), (fp).clone()) });
    ({ fflush_10((fp).clone()) });
    let mut fd: i32 = ({ fileno_11((fp).clone()) });
    let st: Value<stat> = <Value<stat>>::default();
    {
        &(4usize);
        {
            if (((({ fstat_12(fd, (st.as_pointer())) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"fstat(fd, &st) == 0"),
                        Ptr::<i8>::from_string_literal(b"sys_stat.c"),
                        28_u32,
                        Ptr::<i8>::from_string_literal(b"void test_fstat(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((({ (*st.borrow()).st_size } == 11_i64) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"st.st_size == 11"),
                        Ptr::<i8>::from_string_literal(b"sys_stat.c"),
                        29_u32,
                        Ptr::<i8>::from_string_literal(b"void test_fstat(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((({ (*st.borrow()).st_mtime } > 0_i64) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"st.st_mtime > 0"),
                        Ptr::<i8>::from_string_literal(b"sys_stat.c"),
                        30_u32,
                        Ptr::<i8>::from_string_literal(b"void test_fstat(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ fclose_6((fp).clone()) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"fclose(fp) == 0"),
                        Ptr::<i8>::from_string_literal(b"sys_stat.c"),
                        31_u32,
                        Ptr::<i8>::from_string_literal(b"void test_fstat(void)"),
                    )
                });
            }
        }
    };
    ({ unlink_8((path).clone()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_stat_2() });
    ({ test_fstat_9() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_wide_data;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_codecvt;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_marker;
pub fn fopen_3(mut __filename: Ptr<i8>, mut __modes: Ptr<i8>) -> Ptr<_IO_FILE> {
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
pub fn fputs_5(mut __s: Ptr<i8>, mut __stream: Ptr<_IO_FILE>) -> i32 {
    unimplemented!()
}
pub fn fclose_6(mut __stream: Ptr<_IO_FILE>) -> i32 {
    unimplemented!()
}
pub fn stat_7(mut __file: Ptr<i8>, mut __buf: Ptr<stat>) -> i32 {
    unimplemented!()
}
pub fn unlink_8(mut __name: Ptr<i8>) -> i32 {
    unimplemented!()
}
pub fn fflush_10(mut __stream: Ptr<_IO_FILE>) -> i32 {
    unimplemented!()
}
pub fn fileno_11(mut __stream: Ptr<_IO_FILE>) -> i32 {
    unimplemented!()
}
pub fn fstat_12(mut __fd: i32, mut __buf: Ptr<stat>) -> i32 {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
