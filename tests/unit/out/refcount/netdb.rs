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
pub type __socket_type = u32;
pub const __socket_type_SOCK_STREAM: __socket_type = 1;
pub const __socket_type_SOCK_DGRAM: __socket_type = 2;
pub const __socket_type_SOCK_RAW: __socket_type = 3;
pub const __socket_type_SOCK_RDM: __socket_type = 4;
pub const __socket_type_SOCK_SEQPACKET: __socket_type = 5;
pub const __socket_type_SOCK_DCCP: __socket_type = 6;
pub const __socket_type_SOCK_PACKET: __socket_type = 10;
pub const __socket_type_SOCK_CLOEXEC: __socket_type = 524288;
pub const __socket_type_SOCK_NONBLOCK: __socket_type = 2048;
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
    pub __in6_u: anon_1,
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
pub struct anon_1 {
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
impl Clone for anon_1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_1 {
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
#[byte_size(24)]
pub struct rpcent {
    #[offset(0)]
    #[byte_size(8)]
    pub r_name: Ptr<i8>,
    #[offset(8)]
    #[byte_size(8)]
    pub r_aliases: Ptr<Ptr<i8>>,
    #[offset(16)]
    pub r_number: i32,
}
impl Clone for rpcent {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rpcent {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct netent {
    #[offset(0)]
    #[byte_size(8)]
    pub n_name: Ptr<i8>,
    #[offset(8)]
    #[byte_size(8)]
    pub n_aliases: Ptr<Ptr<i8>>,
    #[offset(16)]
    pub n_addrtype: i32,
    #[offset(20)]
    pub n_net: u32,
}
impl Clone for netent {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for netent {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct hostent {
    #[offset(0)]
    #[byte_size(8)]
    pub h_name: Ptr<i8>,
    #[offset(8)]
    #[byte_size(8)]
    pub h_aliases: Ptr<Ptr<i8>>,
    #[offset(16)]
    pub h_addrtype: i32,
    #[offset(20)]
    pub h_length: i32,
    #[offset(24)]
    #[byte_size(8)]
    pub h_addr_list: Ptr<Ptr<i8>>,
}
impl Clone for hostent {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hostent {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct servent {
    #[offset(0)]
    #[byte_size(8)]
    pub s_name: Ptr<i8>,
    #[offset(8)]
    #[byte_size(8)]
    pub s_aliases: Ptr<Ptr<i8>>,
    #[offset(16)]
    pub s_port: i32,
    #[offset(24)]
    #[byte_size(8)]
    pub s_proto: Ptr<i8>,
}
impl Clone for servent {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for servent {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct protoent {
    #[offset(0)]
    #[byte_size(8)]
    pub p_name: Ptr<i8>,
    #[offset(8)]
    #[byte_size(8)]
    pub p_aliases: Ptr<Ptr<i8>>,
    #[offset(16)]
    pub p_proto: i32,
}
impl Clone for protoent {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for protoent {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(48)]
pub struct addrinfo {
    #[offset(0)]
    pub ai_flags: i32,
    #[offset(4)]
    pub ai_family: i32,
    #[offset(8)]
    pub ai_socktype: i32,
    #[offset(12)]
    pub ai_protocol: i32,
    #[offset(16)]
    pub ai_addrlen: u32,
    #[offset(24)]
    #[byte_size(8)]
    pub ai_addr: Ptr<sockaddr>,
    #[offset(32)]
    #[byte_size(8)]
    pub ai_canonname: Ptr<i8>,
    #[offset(40)]
    #[byte_size(8)]
    pub ai_next: Ptr<addrinfo>,
}
impl Clone for addrinfo {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for addrinfo {
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
pub fn test_ipv4_literal_2() {
    let hints: Value<addrinfo> = <Value<addrinfo>>::default();
    (*hints.borrow_mut()).ai_flags = 0;
    (*hints.borrow_mut()).ai_protocol = 0;
    (*hints.borrow_mut()).ai_addrlen = 0_u32;
    (*hints.borrow_mut()).ai_addr = Ptr::<sockaddr>::null();
    (*hints.borrow_mut()).ai_canonname = Ptr::<i8>::null();
    (*hints.borrow_mut()).ai_next = Ptr::<addrinfo>::null();
    (*hints.borrow_mut()).ai_family = 2;
    (*hints.borrow_mut()).ai_socktype = (__socket_type_SOCK_STREAM as i32);
    let res: Value<Ptr<addrinfo>> = Rc::new(RefCell::new(Ptr::<addrinfo>::null()));
    {
        &(4usize);
        {
            if (((({
                getaddrinfo_3(
                    Ptr::<i8>::from_string_literal(b"127.0.0.1"),
                    Ptr::<i8>::from_string_literal(b"8080"),
                    (hints.as_pointer()),
                    (res.as_pointer()),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(
                            b"getaddrinfo(\"127.0.0.1\", \"8080\", &hints, &res) == 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"netdb.c"),
                        19_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ipv4_literal(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((!((*res.borrow()).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"res != NULL"),
                        Ptr::<i8>::from_string_literal(b"netdb.c"),
                        20_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ipv4_literal(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((*res.borrow()).with(|__s| __s.ai_family) == 2) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"res->ai_family == AF_INET"),
                        Ptr::<i8>::from_string_literal(b"netdb.c"),
                        21_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ipv4_literal(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((*res.borrow()).with(|__s| __s.ai_socktype) == (__socket_type_SOCK_STREAM as i32))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"res->ai_socktype == SOCK_STREAM"),
                        Ptr::<i8>::from_string_literal(b"netdb.c"),
                        22_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ipv4_literal(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((((*res.borrow()).with(|__s| __s.ai_addrlen) as usize) == 16usize) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(
                            b"res->ai_addrlen == sizeof(struct sockaddr_in)",
                        ),
                        Ptr::<i8>::from_string_literal(b"netdb.c"),
                        23_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ipv4_literal(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((!(((*res.borrow()).with(|__s| __s.ai_addr.clone())).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"res->ai_addr != NULL"),
                        Ptr::<i8>::from_string_literal(b"netdb.c"),
                        24_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ipv4_literal(void)"),
                    )
                });
            }
        }
    };
    let mut sin: Ptr<sockaddr_in> = (*res.borrow())
        .with(|__s| __s.ai_addr.clone())
        .reinterpret_cast::<sockaddr_in>();
    {
        &(4usize);
        {
            if ((((sin.with(|__s| __s.sin_family) as i32) == 2) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"sin->sin_family == AF_INET"),
                        Ptr::<i8>::from_string_literal(b"netdb.c"),
                        26_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ipv4_literal(void)"),
                    )
                });
            }
        }
    };
    let port_be: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        ((8080 / 256) as u8),
        ((8080 % 256) as u8),
    ])));
    {
        &(4usize);
        {
            if (((({
                memcmp_5(
                    ((field_ptr!(sin, sin_port)) as Ptr<u16>).to_any(),
                    ((port_be.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                    2_usize,
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"memcmp(&sin->sin_port, port_be, 2) == 0"),
                        Ptr::<i8>::from_string_literal(b"netdb.c"),
                        28_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ipv4_literal(void)"),
                    )
                });
            }
        }
    };
    let addr_be: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([127_u8, 0_u8, 0_u8, 1_u8])));
    {
        &(4usize);
        {
            if (((({
                memcmp_5(
                    ((field_ptr!(sin, sin_addr)) as Ptr<in_addr>).to_any(),
                    ((addr_be.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                    4_usize,
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"memcmp(&sin->sin_addr, addr_be, 4) == 0"),
                        Ptr::<i8>::from_string_literal(b"netdb.c"),
                        30_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ipv4_literal(void)"),
                    )
                });
            }
        }
    };
    ({ freeaddrinfo_6((*res.borrow()).clone()) });
}
pub fn test_ipv6_literal_7() {
    let hints: Value<addrinfo> = <Value<addrinfo>>::default();
    (*hints.borrow_mut()).ai_flags = 0;
    (*hints.borrow_mut()).ai_protocol = 0;
    (*hints.borrow_mut()).ai_addrlen = 0_u32;
    (*hints.borrow_mut()).ai_addr = Ptr::<sockaddr>::null();
    (*hints.borrow_mut()).ai_canonname = Ptr::<i8>::null();
    (*hints.borrow_mut()).ai_next = Ptr::<addrinfo>::null();
    (*hints.borrow_mut()).ai_family = 10;
    (*hints.borrow_mut()).ai_socktype = (__socket_type_SOCK_STREAM as i32);
    let res: Value<Ptr<addrinfo>> = Rc::new(RefCell::new(Ptr::<addrinfo>::null()));
    {
        &(4usize);
        {
            if (((({
                getaddrinfo_3(
                    Ptr::<i8>::from_string_literal(b"::1"),
                    Ptr::<i8>::from_string_literal(b"443"),
                    (hints.as_pointer()),
                    (res.as_pointer()),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(
                            b"getaddrinfo(\"::1\", \"443\", &hints, &res) == 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"netdb.c"),
                        45_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ipv6_literal(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((!((*res.borrow()).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"res != NULL"),
                        Ptr::<i8>::from_string_literal(b"netdb.c"),
                        46_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ipv6_literal(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((*res.borrow()).with(|__s| __s.ai_family) == 10) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"res->ai_family == AF_INET6"),
                        Ptr::<i8>::from_string_literal(b"netdb.c"),
                        47_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ipv6_literal(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((((*res.borrow()).with(|__s| __s.ai_addrlen) as usize) == 28usize) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(
                            b"res->ai_addrlen == sizeof(struct sockaddr_in6)",
                        ),
                        Ptr::<i8>::from_string_literal(b"netdb.c"),
                        48_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ipv6_literal(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((!(((*res.borrow()).with(|__s| __s.ai_addr.clone())).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"res->ai_addr != NULL"),
                        Ptr::<i8>::from_string_literal(b"netdb.c"),
                        49_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ipv6_literal(void)"),
                    )
                });
            }
        }
    };
    let mut sin6: Ptr<sockaddr_in6> = (*res.borrow())
        .with(|__s| __s.ai_addr.clone())
        .reinterpret_cast::<sockaddr_in6>();
    {
        &(4usize);
        {
            if ((((sin6.with(|__s| __s.sin6_family) as i32) == 10) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"sin6->sin6_family == AF_INET6"),
                        Ptr::<i8>::from_string_literal(b"netdb.c"),
                        51_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ipv6_literal(void)"),
                    )
                });
            }
        }
    };
    let port_be: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        ((443 / 256) as u8),
        ((443 % 256) as u8),
    ])));
    {
        &(4usize);
        {
            if (((({
                memcmp_5(
                    ((field_ptr!(sin6, sin6_port)) as Ptr<u16>).to_any(),
                    ((port_be.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                    2_usize,
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(
                            b"memcmp(&sin6->sin6_port, port_be, 2) == 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"netdb.c"),
                        53_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ipv6_literal(void)"),
                    )
                });
            }
        }
    };
    let addr_be: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        1_u8,
    ])));
    {
        &(4usize);
        {
            if (((({
                memcmp_5(
                    ((field_ptr!(sin6, sin6_addr)) as Ptr<in6_addr>).to_any(),
                    ((addr_be.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                    16_usize,
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(
                            b"memcmp(&sin6->sin6_addr, addr_be, 16) == 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"netdb.c"),
                        55_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ipv6_literal(void)"),
                    )
                });
            }
        }
    };
    ({ freeaddrinfo_6((*res.borrow()).clone()) });
}
pub fn test_null_hints_8() {
    let res: Value<Ptr<addrinfo>> = Rc::new(RefCell::new(Ptr::<addrinfo>::null()));
    {
        &(4usize);
        {
            if (((({
                getaddrinfo_3(
                    Ptr::<i8>::from_string_literal(b"127.0.0.1"),
                    Ptr::<i8>::from_string_literal(b"80"),
                    Ptr::<addrinfo>::null(),
                    (res.as_pointer()),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(
                            b"getaddrinfo(\"127.0.0.1\", \"80\", NULL, &res) == 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"netdb.c"),
                        61_u32,
                        Ptr::<i8>::from_string_literal(b"void test_null_hints(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((!((*res.borrow()).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"res != NULL"),
                        Ptr::<i8>::from_string_literal(b"netdb.c"),
                        62_u32,
                        Ptr::<i8>::from_string_literal(b"void test_null_hints(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((*res.borrow()).with(|__s| __s.ai_family) == 2) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"res->ai_family == AF_INET"),
                        Ptr::<i8>::from_string_literal(b"netdb.c"),
                        63_u32,
                        Ptr::<i8>::from_string_literal(b"void test_null_hints(void)"),
                    )
                });
            }
        }
    };
    let mut sin: Ptr<sockaddr_in> = (*res.borrow())
        .with(|__s| __s.ai_addr.clone())
        .reinterpret_cast::<sockaddr_in>();
    let addr_be: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([127_u8, 0_u8, 0_u8, 1_u8])));
    {
        &(4usize);
        {
            if (((({
                memcmp_5(
                    ((field_ptr!(sin, sin_addr)) as Ptr<in_addr>).to_any(),
                    ((addr_be.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                    4_usize,
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"memcmp(&sin->sin_addr, addr_be, 4) == 0"),
                        Ptr::<i8>::from_string_literal(b"netdb.c"),
                        66_u32,
                        Ptr::<i8>::from_string_literal(b"void test_null_hints(void)"),
                    )
                });
            }
        }
    };
    ({ freeaddrinfo_6((*res.borrow()).clone()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_ipv4_literal_2() });
    ({ test_ipv6_literal_7() });
    ({ test_null_hints_8() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __locale_data;
pub fn getaddrinfo_3(
    mut __name: Ptr<i8>,
    mut __service: Ptr<i8>,
    mut __req: Ptr<addrinfo>,
    mut __pai: Ptr<Ptr<addrinfo>>,
) -> i32 {
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
pub fn memcmp_5(mut __s1: AnyPtr, mut __s2: AnyPtr, mut __n: usize) -> i32 {
    unimplemented!()
}
pub fn freeaddrinfo_6(mut __ai: Ptr<addrinfo>) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
