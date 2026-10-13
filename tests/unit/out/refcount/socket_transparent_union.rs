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
#[byte_size(16)]
pub struct iovec {
    #[offset(0)]
    #[byte_size(8)]
    pub iov_base: AnyPtr,
    #[offset(8)]
    pub iov_len: usize,
}
impl Clone for iovec {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iovec {
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
    pub fds_bits: Value<Box<[i64]>>,
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
pub struct __atomic_wide_counter {
    #[offset(0)]
    pub __value64: u64,
    #[offset(0)]
    #[byte_size(8)]
    pub __value32: anon_0,
}
impl Clone for __atomic_wide_counter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __atomic_wide_counter {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    pub __low: u32,
    #[offset(4)]
    pub __high: u32,
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
pub struct __pthread_internal_list {
    #[offset(0)]
    #[byte_size(8)]
    pub __prev: Ptr<__pthread_internal_list>,
    #[offset(8)]
    #[byte_size(8)]
    pub __next: Ptr<__pthread_internal_list>,
}
impl Clone for __pthread_internal_list {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_internal_list {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __pthread_internal_slist {
    #[offset(0)]
    #[byte_size(8)]
    pub __next: Ptr<__pthread_internal_slist>,
}
impl Clone for __pthread_internal_slist {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_internal_slist {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(40)]
pub struct __pthread_mutex_s {
    #[offset(0)]
    pub __lock: i32,
    #[offset(4)]
    pub __count: u32,
    #[offset(8)]
    pub __owner: i32,
    #[offset(12)]
    pub __nusers: u32,
    #[offset(16)]
    pub __kind: i32,
    #[offset(20)]
    pub __spins: i16,
    #[offset(22)]
    pub __elision: i16,
    #[offset(24)]
    #[byte_size(16)]
    pub __list: __pthread_internal_list,
}
impl Clone for __pthread_mutex_s {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_mutex_s {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct __pthread_rwlock_arch_t {
    #[offset(0)]
    pub __readers: u32,
    #[offset(4)]
    pub __writers: u32,
    #[offset(8)]
    pub __wrphase_futex: u32,
    #[offset(12)]
    pub __writers_futex: u32,
    #[offset(16)]
    pub __pad3: u32,
    #[offset(20)]
    pub __pad4: u32,
    #[offset(24)]
    pub __cur_writer: i32,
    #[offset(28)]
    pub __shared: i32,
    #[offset(32)]
    pub __rwelision: i8,
    #[offset(33)]
    #[byte_size(7)]
    pub __pad1: Value<Box<[u8]>>,
    #[offset(40)]
    pub __pad2: u64,
    #[offset(48)]
    pub __flags: u32,
}
impl Clone for __pthread_rwlock_arch_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_rwlock_arch_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(48)]
pub struct __pthread_cond_s {
    #[offset(0)]
    #[byte_size(8)]
    pub __wseq: __atomic_wide_counter,
    #[offset(8)]
    #[byte_size(8)]
    pub __g1_start: __atomic_wide_counter,
    #[offset(16)]
    #[byte_size(8)]
    pub __g_refs: Value<Box<[u32]>>,
    #[offset(24)]
    #[byte_size(8)]
    pub __g_size: Value<Box<[u32]>>,
    #[offset(32)]
    pub __g1_orig_size: u32,
    #[offset(36)]
    pub __wrefs: u32,
    #[offset(40)]
    #[byte_size(8)]
    pub __g_signals: Value<Box<[u32]>>,
}
impl Clone for __pthread_cond_s {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_cond_s {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct __once_flag {
    #[offset(0)]
    pub __data: i32,
}
impl Clone for __once_flag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __once_flag {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct pthread_mutexattr_t {
    #[offset(0)]
    #[byte_size(4)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i32,
}
impl Clone for pthread_mutexattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_mutexattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct pthread_condattr_t {
    #[offset(0)]
    #[byte_size(4)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i32,
}
impl Clone for pthread_condattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_condattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct pthread_attr_t {
    #[offset(0)]
    #[byte_size(56)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_attr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_attr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(40)]
pub struct pthread_mutex_t {
    #[offset(0)]
    #[byte_size(40)]
    pub __data: __pthread_mutex_s,
    #[offset(0)]
    #[byte_size(40)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_mutex_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_mutex_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(48)]
pub struct pthread_cond_t {
    #[offset(0)]
    #[byte_size(48)]
    pub __data: __pthread_cond_s,
    #[offset(0)]
    #[byte_size(48)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_cond_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_cond_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct pthread_rwlock_t {
    #[offset(0)]
    #[byte_size(56)]
    pub __data: __pthread_rwlock_arch_t,
    #[offset(0)]
    #[byte_size(56)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_rwlock_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_rwlock_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct pthread_rwlockattr_t {
    #[offset(0)]
    #[byte_size(8)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_rwlockattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_rwlockattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct pthread_barrier_t {
    #[offset(0)]
    #[byte_size(32)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_barrier_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_barrier_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct pthread_barrierattr_t {
    #[offset(0)]
    #[byte_size(4)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i32,
}
impl Clone for pthread_barrierattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_barrierattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct sockaddr {
    #[offset(0)]
    pub sa_family: u16,
    #[offset(2)]
    #[byte_size(14)]
    pub sa_data: Value<Box<[i8]>>,
}
impl Clone for sockaddr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for sockaddr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct sockaddr_storage {
    #[offset(0)]
    pub ss_family: u16,
    #[offset(2)]
    #[byte_size(118)]
    pub __ss_padding: Value<Box<[i8]>>,
    #[offset(120)]
    pub __ss_align: u64,
}
impl Clone for sockaddr_storage {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for sockaddr_storage {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_1 = u32;
pub const anon_1_MSG_OOB: anon_1 = 1;
pub const anon_1_MSG_PEEK: anon_1 = 2;
pub const anon_1_MSG_DONTROUTE: anon_1 = 4;
pub const anon_1_MSG_TRYHARD: anon_1 = 4;
pub const anon_1_MSG_CTRUNC: anon_1 = 8;
pub const anon_1_MSG_PROXY: anon_1 = 16;
pub const anon_1_MSG_TRUNC: anon_1 = 32;
pub const anon_1_MSG_DONTWAIT: anon_1 = 64;
pub const anon_1_MSG_EOR: anon_1 = 128;
pub const anon_1_MSG_WAITALL: anon_1 = 256;
pub const anon_1_MSG_FIN: anon_1 = 512;
pub const anon_1_MSG_SYN: anon_1 = 1024;
pub const anon_1_MSG_CONFIRM: anon_1 = 2048;
pub const anon_1_MSG_RST: anon_1 = 4096;
pub const anon_1_MSG_ERRQUEUE: anon_1 = 8192;
pub const anon_1_MSG_NOSIGNAL: anon_1 = 16384;
pub const anon_1_MSG_MORE: anon_1 = 32768;
pub const anon_1_MSG_WAITFORONE: anon_1 = 65536;
pub const anon_1_MSG_BATCH: anon_1 = 262144;
pub const anon_1_MSG_ZEROCOPY: anon_1 = 67108864;
pub const anon_1_MSG_FASTOPEN: anon_1 = 536870912;
pub const anon_1_MSG_CMSG_CLOEXEC: anon_1 = 1073741824;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct msghdr {
    #[offset(0)]
    #[byte_size(8)]
    pub msg_name: AnyPtr,
    #[offset(8)]
    pub msg_namelen: u32,
    #[offset(16)]
    #[byte_size(8)]
    pub msg_iov: Ptr<iovec>,
    #[offset(24)]
    pub msg_iovlen: usize,
    #[offset(32)]
    #[byte_size(8)]
    pub msg_control: AnyPtr,
    #[offset(40)]
    pub msg_controllen: usize,
    #[offset(48)]
    pub msg_flags: i32,
}
impl Clone for msghdr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for msghdr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct cmsghdr {
    #[offset(0)]
    pub cmsg_len: usize,
    #[offset(8)]
    pub cmsg_level: i32,
    #[offset(12)]
    pub cmsg_type: i32,
    #[offset(16)]
    #[byte_size(0)]
    pub __cmsg_data: Value<Box<[u8]>>,
}
impl Clone for cmsghdr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for cmsghdr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(12)]
pub struct ucred {
    #[offset(0)]
    pub pid: i32,
    #[offset(4)]
    pub uid: u32,
    #[offset(8)]
    pub gid: u32,
}
impl Clone for ucred {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ucred {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct __kernel_fd_set {
    #[offset(0)]
    #[byte_size(128)]
    pub fds_bits: Value<Box<[u64]>>,
}
impl Clone for __kernel_fd_set {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __kernel_fd_set {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __kernel_fsid_t {
    #[offset(0)]
    #[byte_size(8)]
    pub val: Value<Box<[i32]>>,
}
impl Clone for __kernel_fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __kernel_fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct linger {
    #[offset(0)]
    pub l_onoff: i32,
    #[offset(4)]
    pub l_linger: i32,
}
impl Clone for linger {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for linger {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct osockaddr {
    #[offset(0)]
    pub sa_family: u16,
    #[offset(2)]
    #[byte_size(14)]
    pub sa_data: Value<Box<[u8]>>,
}
impl Clone for osockaddr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for osockaddr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __SOCKADDR_ARG {
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr__: Ptr<sockaddr>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_at__: Ptr<sockaddr_at>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_ax25__: Ptr<sockaddr_ax25>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_dl__: Ptr<sockaddr_dl>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_eon__: Ptr<sockaddr_eon>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_in__: Ptr<sockaddr_in>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_in6__: Ptr<sockaddr_in6>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_inarp__: Ptr<sockaddr_inarp>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_ipx__: Ptr<sockaddr_ipx>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_iso__: Ptr<sockaddr_iso>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_ns__: Ptr<sockaddr_ns>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_un__: Ptr<sockaddr_un>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_x25__: Ptr<sockaddr_x25>,
}
impl Clone for __SOCKADDR_ARG {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __SOCKADDR_ARG {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __CONST_SOCKADDR_ARG {
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr__: Ptr<sockaddr>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_at__: Ptr<sockaddr_at>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_ax25__: Ptr<sockaddr_ax25>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_dl__: Ptr<sockaddr_dl>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_eon__: Ptr<sockaddr_eon>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_in__: Ptr<sockaddr_in>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_in6__: Ptr<sockaddr_in6>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_inarp__: Ptr<sockaddr_inarp>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_ipx__: Ptr<sockaddr_ipx>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_iso__: Ptr<sockaddr_iso>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_ns__: Ptr<sockaddr_ns>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_un__: Ptr<sockaddr_un>,
    #[offset(0)]
    #[byte_size(8)]
    pub __sockaddr_x25__: Ptr<sockaddr_x25>,
}
impl Clone for __CONST_SOCKADDR_ARG {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __CONST_SOCKADDR_ARG {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(64)]
pub struct mmsghdr {
    #[offset(0)]
    #[byte_size(56)]
    pub msg_hdr: msghdr,
    #[offset(56)]
    pub msg_len: u32,
}
impl Clone for mmsghdr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for mmsghdr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct in_addr {
    #[offset(0)]
    pub s_addr: u32,
}
impl Clone for in_addr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for in_addr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(44)]
pub struct ip_opts {
    #[offset(0)]
    #[byte_size(4)]
    pub ip_dst: in_addr,
    #[offset(4)]
    #[byte_size(40)]
    pub ip_opts: Value<Box<[i8]>>,
}
impl Clone for ip_opts {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ip_opts {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(12)]
pub struct in_pktinfo {
    #[offset(0)]
    pub ipi_ifindex: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub ipi_spec_dst: in_addr,
    #[offset(8)]
    #[byte_size(4)]
    pub ipi_addr: in_addr,
}
impl Clone for in_pktinfo {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for in_pktinfo {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct in6_addr {
    #[offset(0)]
    #[byte_size(16)]
    pub __in6_u: anon_2,
}
impl Clone for in6_addr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for in6_addr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_2 {
    #[offset(0)]
    #[byte_size(16)]
    pub __u6_addr8: Value<Box<[u8]>>,
    #[offset(0)]
    #[byte_size(16)]
    pub __u6_addr16: Value<Box<[u16]>>,
    #[offset(0)]
    #[byte_size(16)]
    pub __u6_addr32: Value<Box<[u32]>>,
}
impl Clone for anon_2 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_2 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct sockaddr_in {
    #[offset(0)]
    pub sin_family: u16,
    #[offset(2)]
    pub sin_port: u16,
    #[offset(4)]
    #[byte_size(4)]
    pub sin_addr: in_addr,
    #[offset(8)]
    #[byte_size(8)]
    pub sin_zero: Value<Box<[u8]>>,
}
impl Clone for sockaddr_in {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for sockaddr_in {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(28)]
pub struct sockaddr_in6 {
    #[offset(0)]
    pub sin6_family: u16,
    #[offset(2)]
    pub sin6_port: u16,
    #[offset(4)]
    pub sin6_flowinfo: u32,
    #[offset(8)]
    #[byte_size(16)]
    pub sin6_addr: in6_addr,
    #[offset(24)]
    pub sin6_scope_id: u32,
}
impl Clone for sockaddr_in6 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for sockaddr_in6 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct ip_mreq {
    #[offset(0)]
    #[byte_size(4)]
    pub imr_multiaddr: in_addr,
    #[offset(4)]
    #[byte_size(4)]
    pub imr_interface: in_addr,
}
impl Clone for ip_mreq {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ip_mreq {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(12)]
pub struct ip_mreqn {
    #[offset(0)]
    #[byte_size(4)]
    pub imr_multiaddr: in_addr,
    #[offset(4)]
    #[byte_size(4)]
    pub imr_address: in_addr,
    #[offset(8)]
    pub imr_ifindex: i32,
}
impl Clone for ip_mreqn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ip_mreqn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(12)]
pub struct ip_mreq_source {
    #[offset(0)]
    #[byte_size(4)]
    pub imr_multiaddr: in_addr,
    #[offset(4)]
    #[byte_size(4)]
    pub imr_interface: in_addr,
    #[offset(8)]
    #[byte_size(4)]
    pub imr_sourceaddr: in_addr,
}
impl Clone for ip_mreq_source {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ip_mreq_source {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(20)]
pub struct ipv6_mreq {
    #[offset(0)]
    #[byte_size(16)]
    pub ipv6mr_multiaddr: in6_addr,
    #[offset(16)]
    pub ipv6mr_interface: u32,
}
impl Clone for ipv6_mreq {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ipv6_mreq {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(136)]
pub struct group_req {
    #[offset(0)]
    pub gr_interface: u32,
    #[offset(8)]
    #[byte_size(128)]
    pub gr_group: sockaddr_storage,
}
impl Clone for group_req {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for group_req {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(264)]
pub struct group_source_req {
    #[offset(0)]
    pub gsr_interface: u32,
    #[offset(8)]
    #[byte_size(128)]
    pub gsr_group: sockaddr_storage,
    #[offset(136)]
    #[byte_size(128)]
    pub gsr_source: sockaddr_storage,
}
impl Clone for group_source_req {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for group_source_req {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(20)]
pub struct ip_msfilter {
    #[offset(0)]
    #[byte_size(4)]
    pub imsf_multiaddr: in_addr,
    #[offset(4)]
    #[byte_size(4)]
    pub imsf_interface: in_addr,
    #[offset(8)]
    pub imsf_fmode: u32,
    #[offset(12)]
    pub imsf_numsrc: u32,
    #[offset(16)]
    #[byte_size(4)]
    pub imsf_slist: Value<Box<[in_addr]>>,
}
impl Clone for ip_msfilter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ip_msfilter {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(272)]
pub struct group_filter {
    #[offset(0)]
    pub gf_interface: u32,
    #[offset(8)]
    #[byte_size(128)]
    pub gf_group: sockaddr_storage,
    #[offset(136)]
    pub gf_fmode: u32,
    #[offset(140)]
    pub gf_numsrc: u32,
    #[offset(144)]
    #[byte_size(128)]
    pub gf_slist: Value<Box<[sockaddr_storage]>>,
}
impl Clone for group_filter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for group_filter {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(20)]
pub struct in6_pktinfo {
    #[offset(0)]
    #[byte_size(16)]
    pub ipi6_addr: in6_addr,
    #[offset(16)]
    pub ipi6_ifindex: u32,
}
impl Clone for in6_pktinfo {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for in6_pktinfo {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct ip6_mtuinfo {
    #[offset(0)]
    #[byte_size(28)]
    pub ip6m_addr: sockaddr_in6,
    #[offset(28)]
    pub ip6m_mtu: u32,
}
impl Clone for ip6_mtuinfo {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ip6_mtuinfo {
    fn default() -> Self {
        unimplemented!()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut fd: i32 = 0;
    let ssloc: Value<sockaddr_storage> = <Value<sockaddr_storage>>::default();
    let slen: Value<u32> = Rc::new(RefCell::new((128usize as u32)));
    {
        &(4usize);
        {
            if (((({
                getsockname_3(
                    fd,
                    (ssloc.as_pointer()).reinterpret_cast::<sockaddr>(),
                    (slen.as_pointer()),
                )
            }) == -1_i32) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(
                            b"getsockname(fd, (struct sockaddr *)&ssloc, &slen) == -1",
                        ),
                        Ptr::<i8>::from_string_literal(b"socket_transparent_union.c"),
                        12_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let sin: Value<sockaddr_in> = <Value<sockaddr_in>>::default();
    let inlen: Value<u32> = Rc::new(RefCell::new((16usize as u32)));
    {
        &(4usize);
        {
            if (((({
                getsockname_3(
                    fd,
                    (sin.as_pointer()).reinterpret_cast::<sockaddr>(),
                    (inlen.as_pointer()),
                )
            }) == -1_i32) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(
                            b"getsockname(fd, (struct sockaddr *)&sin, &inlen) == -1",
                        ),
                        Ptr::<i8>::from_string_literal(b"socket_transparent_union.c"),
                        16_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct sockaddr_inarp;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct sockaddr_ns;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct sockaddr_iso;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct sockaddr_ax25;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct sockaddr_ipx;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct sockaddr_eon;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct sockaddr_at;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct sockaddr_un;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct sockaddr_x25;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct sockaddr_dl;
pub fn getsockname_3(mut __fd: i32, __addr: __SOCKADDR_ARG, mut __len: Ptr<u32>) -> i32 {
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
pub fn __cpp2rust_init_globals() {}
