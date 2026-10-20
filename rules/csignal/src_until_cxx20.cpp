#include "../common/init.h"
#include "../common/traits.h"
#include <csignal>
#include <cstdint>
#include <stdlib.h>
#include <utility>

int f31(const char *a0, int a1) { return access(a0, a1); }

int f32(const char *a0) { return acct(a0); }

unsigned int f33(unsigned int a0) { return alarm(a0); }

int f34(void *a0) { return brk(a0); }

int f35(const char *a0) { return chdir(a0); }

int f36(const char *a0, unsigned int a1, unsigned int a2) {
  return chown(a0, a1, a2);
}

int f37(const char *a0) { return chroot(a0); }

int f38(int a0) { return close(a0); }

int f39(unsigned int a0, unsigned int a1, int a2) {
  return close_range(a0, a1, a2);
}

void f40(int a0) { return closefrom(a0); }

size_t f41(int a0, char *a1, size_t a2) { return confstr(a0, a1, a2); }

ssize_t f42(int a0, long *a1, int a2, long *a3, size_t a4, unsigned int a5) {
  return copy_file_range(a0, a1, a2, a3, a4, a5);
}

char *f43(const char *a0, const char *a1) { return crypt(a0, a1); }

int f44(int a0, int a1) { return daemon(a0, a1); }

int f45(int a0) { return dup(a0); }

int f46(int a0, int a1) { return dup2(a0, a1); }

int f47(int a0, int a1, int a2) { return dup3(a0, a1, a2); }

int f48(const char *a0, int a1) { return eaccess(a0, a1); }

void f49() { return endusershell(); }

int f50(const char *a0, int a1) { return euidaccess(a0, a1); }

template <typename... Args>
int f51(const char *a0, const char *a1, Args... args) {
  return execl(a0, a1, args...);
}

template <typename... Args>
int f52(const char *a0, const char *a1, Args... args) {
  return execle(a0, a1, args...);
}

template <typename... Args>
int f53(const char *a0, const char *a1, Args... args) {
  return execlp(a0, a1, args...);
}

int f54(const char *a0, char *const a1[]) { return execv(a0, a1); }

int f55(const char *a0, char *const a1[], char *const a2[]) {
  return execve(a0, a1, a2);
}

int f56(int a0, const char *a1, char *const a2[], char *const a3[], int a4) {
  return execveat(a0, a1, a2, a3, a4);
}

int f57(const char *a0, char *const a1[]) { return execvp(a0, a1); }

int f58(const char *a0, char *const a1[], char *const a2[]) {
  return execvpe(a0, a1, a2);
}

int f59(int a0, const char *a1, int a2, int a3) {
  return faccessat(a0, a1, a2, a3);
}

int f60(int a0) { return fchdir(a0); }

int f61(int a0, unsigned int a1, unsigned int a2) { return fchown(a0, a1, a2); }

int f62(int a0, const char *a1, unsigned int a2, unsigned int a3, int a4) {
  return fchownat(a0, a1, a2, a3, a4);
}

int f63(int a0) { return fdatasync(a0); }

int f64(int a0, char *const a1[], char *const a2[]) {
  return fexecve(a0, a1, a2);
}

int f65() { return fork(); }

long f66(int a0, int a1) { return fpathconf(a0, a1); }

int f67(int a0) { return fsync(a0); }

int f68(int a0, long a1) { return ftruncate(a0, a1); }

int f69(int a0, long a1) { return ftruncate64(a0, a1); }

char *f70() { return get_current_dir_name(); }

char *f71(char *a0, size_t a1) { return getcwd(a0, a1); }

int f72(char *a0, size_t a1) { return getdomainname(a0, a1); }

int f73() { return getdtablesize(); }

unsigned int f74() { return getegid(); }

int f75(void *a0, size_t a1) { return getentropy(a0, a1); }

unsigned int f76() { return geteuid(); }

unsigned int f77() { return getgid(); }

long f78() { return gethostid(); }

int f79(char *a0, size_t a1) { return gethostname(a0, a1); }

char *f80() { return getlogin(); }

int f81(char *a0, size_t a1) { return getlogin_r(a0, a1); }

int f82(int a0, char *const *a1, const char *a2) { return getopt(a0, a1, a2); }

int f83() { return getpagesize(); }

char *f84(const char *a0) { return getpass(a0); }

int f85(int a0) { return getpgid(a0); }

int f86() { return getpgrp(); }

int f87() { return getpid(); }

int f88() { return getppid(); }

int f89(unsigned int *a0, unsigned int *a1, unsigned int *a2) {
  return getresgid(a0, a1, a2);
}

int f90(unsigned int *a0, unsigned int *a1, unsigned int *a2) {
  return getresuid(a0, a1, a2);
}

int f91(int a0) { return getsid(a0); }

int f92() { return gettid(); }

unsigned int f93() { return getuid(); }

char *f94() { return getusershell(); }

int f95(unsigned int a0) { return group_member(a0); }

int f96(int a0) { return isatty(a0); }

int f97(const char *a0, unsigned int a1, unsigned int a2) {
  return lchown(a0, a1, a2);
}

int f98(const char *a0, const char *a1) { return link(a0, a1); }

int f99(int a0, const char *a1, int a2, const char *a3, int a4) {
  return linkat(a0, a1, a2, a3, a4);
}

int f100(int a0, int a1, long a2) { return lockf(a0, a1, a2); }

int f101(int a0, int a1, long a2) { return lockf64(a0, a1, a2); }

long f102(int a0, long a1, int a2) { return lseek(a0, a1, a2); }

long f103(int a0, long a1, int a2) { return lseek64(a0, a1, a2); }

int f104(int a0) { return nice(a0); }

long f105(const char *a0, int a1) { return pathconf(a0, a1); }

int f106() { return pause(); }

int f107(int a0[2]) { return pipe(a0); }

int f108(int a0[2], int a1) { return pipe2(a0, a1); }

ssize_t f109(int a0, void *a1, size_t a2, long a3) {
  return pread(a0, a1, a2, a3);
}

ssize_t f110(int a0, void *a1, size_t a2, long a3) {
  return pread64(a0, a1, a2, a3);
}

int f111(unsigned short *a0, size_t a1, size_t a2, unsigned int a3) {
  return profil(a0, a1, a2, a3);
}

ssize_t f112(int a0, const void *a1, size_t a2, long a3) {
  return pwrite(a0, a1, a2, a3);
}

ssize_t f113(int a0, const void *a1, size_t a2, long a3) {
  return pwrite64(a0, a1, a2, a3);
}

ssize_t f114(int a0, void *a1, size_t a2) { return read(a0, a1, a2); }

ssize_t f115(const char *__restrict a0, char *__restrict a1, size_t a2) {
  return readlink(a0, a1, a2);
}

ssize_t f116(int a0, const char *__restrict a1, char *__restrict a2,
             size_t a3) {
  return readlinkat(a0, a1, a2, a3);
}

int f117(const char *a0) { return revoke(a0); }

int f118(const char *a0) { return rmdir(a0); }

void *f119(intptr_t a0) { return sbrk(a0); }

int f120(const char *a0, size_t a1) { return setdomainname(a0, a1); }

int f121(unsigned int a0) { return setegid(a0); }

int f122(unsigned int a0) { return seteuid(a0); }

int f123(unsigned int a0) { return setgid(a0); }

int f124(long a0) { return sethostid(a0); }

int f125(const char *a0, size_t a1) { return sethostname(a0, a1); }

int f126(const char *a0) { return setlogin(a0); }

int f127(int a0, int a1) { return setpgid(a0, a1); }

int f128() { return setpgrp(); }

int f129(unsigned int a0, unsigned int a1) { return setregid(a0, a1); }

int f130(unsigned int a0, unsigned int a1, unsigned int a2) {
  return setresgid(a0, a1, a2);
}

int f131(unsigned int a0, unsigned int a1, unsigned int a2) {
  return setresuid(a0, a1, a2);
}

int f132(unsigned int a0, unsigned int a1) { return setreuid(a0, a1); }

int f133() { return setsid(); }

int f134(unsigned int a0) { return setuid(a0); }

void f135() { return setusershell(); }

unsigned int f136(unsigned int a0) { return sleep(a0); }

using t19 = socklen_t;

void f137(const void *__restrict a0, void *__restrict a1, ssize_t a2) {
  return swab(a0, a1, a2);
}

int f138(const char *a0, const char *a1) { return symlink(a0, a1); }

int f139(const char *a0, int a1, const char *a2) {
  return symlinkat(a0, a1, a2);
}

void f140() { return sync(); }

int f141(int a0) { return syncfs(a0); }

template <typename... Args> long f142(long a0, Args... args) {
  return syscall(a0, args...);
}

long f143(int a0) { return sysconf(a0); }

int f144(int a0) { return tcgetpgrp(a0); }

int f145(int a0, int a1) { return tcsetpgrp(a0, a1); }

int f146(const char *a0, long a1) { return truncate(a0, a1); }

int f147(const char *a0, long a1) { return truncate64(a0, a1); }

char *f148(int a0) { return ttyname(a0); }

int f149(int a0, char *a1, size_t a2) { return ttyname_r(a0, a1, a2); }

int f150() { return ttyslot(); }

unsigned int f151(unsigned int a0, unsigned int a1) { return ualarm(a0, a1); }

int f152(const char *a0) { return unlink(a0); }

int f153(int a0, const char *a1, int a2) { return unlinkat(a0, a1, a2); }

int f154(unsigned int a0) { return usleep(a0); }

int f155() { return vfork(); }

int f156() { return vhangup(); }

ssize_t f157(int a0, const void *a1, size_t a2) { return write(a0, a1, a2); }
