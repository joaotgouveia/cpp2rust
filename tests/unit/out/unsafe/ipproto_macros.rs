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
pub struct iovec {
    pub iov_base: *mut ::libc::c_void,
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
#[derive(VaArg, FnPtrArg)]
pub struct __sigset_t {
    pub __val: [u64; 16],
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
#[derive(VaArg, FnPtrArg)]
pub struct timeval {
    pub tv_sec: i64,
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
#[derive(VaArg, FnPtrArg)]
pub struct timespec {
    pub tv_sec: i64,
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
#[derive(VaArg, FnPtrArg)]
pub struct fd_set {
    pub fds_bits: [i64; 16],
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
#[derive(VaArg, FnPtrArg)]
pub struct __atomic_wide_counter {
    pub __value64: u64,
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
#[derive(VaArg, FnPtrArg)]
pub struct anon_0 {
    pub __low: u32,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_internal_slist {
    pub __next: *mut __pthread_internal_slist,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_mutex_s {
    pub __lock: i32,
    pub __count: u32,
    pub __owner: i32,
    pub __nusers: u32,
    pub __kind: i32,
    pub __spins: i16,
    pub __elision: i16,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: u32,
    pub __writers: u32,
    pub __wrphase_futex: u32,
    pub __writers_futex: u32,
    pub __pad3: u32,
    pub __pad4: u32,
    pub __cur_writer: i32,
    pub __shared: i32,
    pub __rwelision: i8,
    pub __pad1: [u8; 7],
    pub __pad2: u64,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [u32; 2],
    pub __g_size: [u32; 2],
    pub __g1_orig_size: u32,
    pub __wrefs: u32,
    pub __g_signals: [u32; 2],
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
#[derive(VaArg, FnPtrArg)]
pub struct __once_flag {
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_condattr_t {
    pub __size: [libc::c_char; 4],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_attr_t {
    pub __size: [libc::c_char; 56],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [libc::c_char; 56],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_rwlockattr_t {
    pub __size: [libc::c_char; 8],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_barrier_t {
    pub __size: [libc::c_char; 32],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_barrierattr_t {
    pub __size: [libc::c_char; 4],
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
#[derive(VaArg, FnPtrArg)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [libc::c_char; 14],
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
#[derive(VaArg, FnPtrArg)]
pub struct sockaddr_storage {
    pub ss_family: u16,
    pub __ss_padding: [libc::c_char; 118],
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
#[derive(VaArg, FnPtrArg)]
pub struct msghdr {
    pub msg_name: *mut ::libc::c_void,
    pub msg_namelen: u32,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: usize,
    pub msg_control: *mut ::libc::c_void,
    pub msg_controllen: usize,
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
#[derive(VaArg, FnPtrArg)]
pub struct cmsghdr {
    pub cmsg_len: usize,
    pub cmsg_level: i32,
    pub cmsg_type: i32,
    pub __cmsg_data: [u8; 0],
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
#[derive(VaArg, FnPtrArg)]
pub struct ucred {
    pub pid: i32,
    pub uid: u32,
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
#[derive(VaArg, FnPtrArg)]
pub struct __kernel_fd_set {
    pub fds_bits: [u64; 16],
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
#[derive(VaArg, FnPtrArg)]
pub struct __kernel_fsid_t {
    pub val: [i32; 2],
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
#[derive(VaArg, FnPtrArg)]
pub struct linger {
    pub l_onoff: i32,
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
#[derive(VaArg, FnPtrArg)]
pub struct osockaddr {
    pub sa_family: u16,
    pub sa_data: [u8; 14],
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
#[derive(VaArg, FnPtrArg)]
pub struct mmsghdr {
    pub msg_hdr: msghdr,
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
#[derive(VaArg, FnPtrArg)]
pub struct in_addr {
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
#[derive(VaArg, FnPtrArg)]
pub struct ip_opts {
    pub ip_dst: in_addr,
    pub ip_opts: [libc::c_char; 40],
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
#[derive(VaArg, FnPtrArg)]
pub struct in_pktinfo {
    pub ipi_ifindex: i32,
    pub ipi_spec_dst: in_addr,
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
pub type anon_2 = u32;
pub const anon_2_IPPROTO_IP: anon_2 = 0;
pub const anon_2_IPPROTO_ICMP: anon_2 = 1;
pub const anon_2_IPPROTO_IGMP: anon_2 = 2;
pub const anon_2_IPPROTO_IPIP: anon_2 = 4;
pub const anon_2_IPPROTO_TCP: anon_2 = 6;
pub const anon_2_IPPROTO_EGP: anon_2 = 8;
pub const anon_2_IPPROTO_PUP: anon_2 = 12;
pub const anon_2_IPPROTO_UDP: anon_2 = 17;
pub const anon_2_IPPROTO_IDP: anon_2 = 22;
pub const anon_2_IPPROTO_TP: anon_2 = 29;
pub const anon_2_IPPROTO_DCCP: anon_2 = 33;
pub const anon_2_IPPROTO_IPV6: anon_2 = 41;
pub const anon_2_IPPROTO_RSVP: anon_2 = 46;
pub const anon_2_IPPROTO_GRE: anon_2 = 47;
pub const anon_2_IPPROTO_ESP: anon_2 = 50;
pub const anon_2_IPPROTO_AH: anon_2 = 51;
pub const anon_2_IPPROTO_MTP: anon_2 = 92;
pub const anon_2_IPPROTO_BEETPH: anon_2 = 94;
pub const anon_2_IPPROTO_ENCAP: anon_2 = 98;
pub const anon_2_IPPROTO_PIM: anon_2 = 103;
pub const anon_2_IPPROTO_COMP: anon_2 = 108;
pub const anon_2_IPPROTO_L2TP: anon_2 = 115;
pub const anon_2_IPPROTO_SCTP: anon_2 = 132;
pub const anon_2_IPPROTO_UDPLITE: anon_2 = 136;
pub const anon_2_IPPROTO_MPLS: anon_2 = 137;
pub const anon_2_IPPROTO_ETHERNET: anon_2 = 143;
pub const anon_2_IPPROTO_RAW: anon_2 = 255;
pub const anon_2_IPPROTO_MPTCP: anon_2 = 262;
pub const anon_2_IPPROTO_MAX: anon_2 = 263;
#[derive(VaArg, FnPtrArg)]
pub struct in6_addr {
    pub __in6_u: anon_3,
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
#[derive(VaArg, FnPtrArg)]
pub struct anon_3 {
    pub __u6_addr8: [u8; 16],
    pub __u6_addr16: [u16; 8],
    pub __u6_addr32: [u32; 4],
}
impl Clone for anon_3 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_3 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
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
#[derive(VaArg, FnPtrArg)]
pub struct sockaddr_in6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
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
#[derive(VaArg, FnPtrArg)]
pub struct ip_mreq {
    pub imr_multiaddr: in_addr,
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
#[derive(VaArg, FnPtrArg)]
pub struct ip_mreqn {
    pub imr_multiaddr: in_addr,
    pub imr_address: in_addr,
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
#[derive(VaArg, FnPtrArg)]
pub struct ip_mreq_source {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
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
#[derive(VaArg, FnPtrArg)]
pub struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
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
#[derive(VaArg, FnPtrArg)]
pub struct group_req {
    pub gr_interface: u32,
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
#[derive(VaArg, FnPtrArg)]
pub struct group_source_req {
    pub gsr_interface: u32,
    pub gsr_group: sockaddr_storage,
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
#[derive(VaArg, FnPtrArg)]
pub struct ip_msfilter {
    pub imsf_multiaddr: in_addr,
    pub imsf_interface: in_addr,
    pub imsf_fmode: u32,
    pub imsf_numsrc: u32,
    pub imsf_slist: [in_addr; 1],
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
#[derive(VaArg, FnPtrArg)]
pub struct group_filter {
    pub gf_interface: u32,
    pub gf_group: sockaddr_storage,
    pub gf_fmode: u32,
    pub gf_numsrc: u32,
    pub gf_slist: [sockaddr_storage; 1],
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
#[derive(VaArg, FnPtrArg)]
pub struct in6_pktinfo {
    pub ipi6_addr: in6_addr,
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
#[derive(VaArg, FnPtrArg)]
pub struct ip6_mtuinfo {
    pub ip6m_addr: sockaddr_in6,
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
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut tcp: i32 = (anon_2_IPPROTO_TCP as i32);
    let mut udp: i32 = (anon_2_IPPROTO_UDP as i32);
    let mut ip: i32 = (anon_2_IPPROTO_IP as i32);
    let mut ip6: i32 = (anon_2_IPPROTO_IPV6 as i32);
    (if (((((tcp) + (udp)) + (ip)) + (ip6)) == (64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"tcp + udp + ip + ip6 == 64".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_4(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
