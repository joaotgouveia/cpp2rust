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
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new((0..16).map(|_| 0_u8).collect::<Box<[u8]>>()));
    {
        &(4usize);
        {
            if (((({
                inet_pton_2(
                    2,
                    Ptr::<i8>::from_string_literal(b"1.2.3.4"),
                    ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                )
            }) == 1) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(
                            b"inet_pton(AF_INET, \"1.2.3.4\", buf) == 1",
                        ),
                        Ptr::<i8>::from_string_literal(b"inet_pton_ntop.c"),
                        7_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((((((((((((*buf.borrow())[(0) as usize] as i32) == 1) as i32) != 0)
                && (((((*buf.borrow())[(1) as usize] as i32) == 2) as i32) != 0))
                as i32)
                != 0)
                && (((((*buf.borrow())[(2) as usize] as i32) == 3) as i32) != 0))
                as i32)
                != 0)
                && (((((*buf.borrow())[(3) as usize] as i32) == 4) as i32) != 0))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(
                            b"buf[0] == 1 && buf[1] == 2 && buf[2] == 3 && buf[3] == 4",
                        ),
                        Ptr::<i8>::from_string_literal(b"inet_pton_ntop.c"),
                        8_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                inet_pton_2(
                    2,
                    Ptr::<i8>::from_string_literal(b"999.1.1.1"),
                    ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(
                            b"inet_pton(AF_INET, \"999.1.1.1\", buf) == 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"inet_pton_ntop.c"),
                        9_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                inet_pton_2(
                    2,
                    Ptr::<i8>::from_string_literal(b"not an ip"),
                    ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(
                            b"inet_pton(AF_INET, \"not an ip\", buf) == 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"inet_pton_ntop.c"),
                        10_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                inet_pton_2(
                    10,
                    Ptr::<i8>::from_string_literal(b"::1"),
                    ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                )
            }) == 1) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"inet_pton(AF_INET6, \"::1\", buf) == 1"),
                        Ptr::<i8>::from_string_literal(b"inet_pton_ntop.c"),
                        11_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((((((*buf.borrow())[(0) as usize] as i32) == 0) as i32) != 0)
                && (((((*buf.borrow())[(15) as usize] as i32) == 1) as i32) != 0))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"buf[0] == 0 && buf[15] == 1"),
                        Ptr::<i8>::from_string_literal(b"inet_pton_ntop.c"),
                        12_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                inet_pton_2(
                    10,
                    Ptr::<i8>::from_string_literal(b"2001:db8::5"),
                    ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                )
            }) == 1) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(
                            b"inet_pton(AF_INET6, \"2001:db8::5\", buf) == 1",
                        ),
                        Ptr::<i8>::from_string_literal(b"inet_pton_ntop.c"),
                        13_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((((((((((*buf.borrow())[(0) as usize] as i32) == 32) as i32) != 0)
                && (((((*buf.borrow())[(1) as usize] as i32) == 1) as i32) != 0))
                as i32)
                != 0)
                && (((((*buf.borrow())[(15) as usize] as i32) == 5) as i32) != 0))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(
                            b"buf[0] == 0x20 && buf[1] == 0x01 && buf[15] == 5",
                        ),
                        Ptr::<i8>::from_string_literal(b"inet_pton_ntop.c"),
                        14_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let text: Value<Box<[i8]>> =
        Rc::new(RefCell::new((0..64).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let four: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([10_u8, 0_u8, 0_u8, 1_u8])));
    {
        &(4usize);
        {
            if (((({
                strcmp_4(
                    ({
                        let ___buf: Ptr<i8> = (text.as_pointer() as Ptr<i8>);
                        let ___len: u32 = (64usize as u32);
                        inet_ntop_5(
                            2,
                            ((four.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                            ___buf,
                            ___len,
                        )
                    }),
                    Ptr::<i8>::from_string_literal(b"10.0.0.1"),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_3 ( Ptr::<i8 >::from_string_literal(b"strcmp(inet_ntop(AF_INET, four, text, sizeof(text)), \"10.0.0.1\") == 0" )  , Ptr::<i8 >::from_string_literal(b"inet_pton_ntop.c" )  , 18_u32  , Ptr::<i8 >::from_string_literal(b"int main(void)" )  , )
                });
            }
        }
    };
    let sixteen: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8,
    ])));
    (*sixteen.borrow_mut())[(15) as usize] = 1_u8;
    {
        &(4usize);
        {
            if (((({
                strcmp_4(
                    ({
                        let ___buf: Ptr<i8> = (text.as_pointer() as Ptr<i8>);
                        let ___len: u32 = (64usize as u32);
                        inet_ntop_5(
                            10,
                            ((sixteen.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                            ___buf,
                            ___len,
                        )
                    }),
                    Ptr::<i8>::from_string_literal(b"::1"),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_3 ( Ptr::<i8 >::from_string_literal(b"strcmp(inet_ntop(AF_INET6, sixteen, text, sizeof(text)), \"::1\") == 0" )  , Ptr::<i8 >::from_string_literal(b"inet_pton_ntop.c" )  , 21_u32  , Ptr::<i8 >::from_string_literal(b"int main(void)" )  , )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                inet_ntop_5(
                    2,
                    ((four.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                    (text.as_pointer() as Ptr<i8>),
                    4_u32,
                )
            })
            .is_null()) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"inet_ntop(AF_INET, four, text, 4) == 0"),
                        Ptr::<i8>::from_string_literal(b"inet_pton_ntop.c"),
                        22_u32,
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
pub struct __locale_data;
pub fn inet_pton_2(mut __af: i32, mut __cp: Ptr<i8>, mut __buf: AnyPtr) -> i32 {
    unimplemented!()
}
pub fn __assert_fail_3(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn strcmp_4(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>) -> i32 {
    unimplemented!()
}
pub fn inet_ntop_5(mut __af: i32, mut __cp: AnyPtr, mut __buf: Ptr<i8>, mut __len: u32) -> Ptr<i8> {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
