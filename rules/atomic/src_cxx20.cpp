#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <atomic>
#include <chrono>
#include <memory>
#include <stdlib.h>
#include <utility>

int f214(const char *a0, int a1) { return access(a0, a1); }

int f215(const char *a0) { return acct(a0); }

unsigned int f216(unsigned int a0) { return alarm(a0); }

char *f217(const struct tm *a0) { return asctime(a0); }

char *f218(const struct tm *__restrict a0, char *__restrict a1) {
  return asctime_r(a0, a1);
}

int f219(void *a0) { return brk(a0); }

int f220(const char *a0) { return chdir(a0); }

int f221(const char *a0, unsigned int a1, unsigned int a2) {
  return chown(a0, a1, a2);
}

int f222(const char *a0) { return chroot(a0); }

clock_t f223() { return clock(); }

int f224(int a0, struct timex *a1) { return clock_adjtime(a0, a1); }

int f225(pid_t a0, clockid_t *a1) { return clock_getcpuclockid(a0, a1); }

int f226(clockid_t a0, struct timespec *a1) { return clock_getres(a0, a1); }

int f227(clockid_t a0, struct timespec *a1) { return clock_gettime(a0, a1); }

int f228(clockid_t a0, int a1, const struct timespec *a2, struct timespec *a3) {
  return clock_nanosleep(a0, a1, a2, a3);
}

int f229(clockid_t a0, const struct timespec *a1) {
  return clock_settime(a0, a1);
}

template <typename... Args>
int f230(int (*a0)(void *), void *a1, int a2, void *a3, Args... args) {
  return clone(a0, a1, a2, a3, args...);
}

int f231(int a0) { return close(a0); }

int f232(unsigned int a0, unsigned int a1, int a2) {
  return close_range(a0, a1, a2);
}

void f233(int a0) { return closefrom(a0); }

size_t f234(int a0, char *a1, size_t a2) { return confstr(a0, a1, a2); }

ssize_t f235(int a0, long *a1, int a2, long *a3, size_t a4, unsigned int a5) {
  return copy_file_range(a0, a1, a2, a3, a4, a5);
}

using t67 = cpu_set_t;

char *f236(const char *a0, const char *a1) { return crypt(a0, a1); }

char *f237(const time_t *a0) { return ctime(a0); }

char *f238(const time_t *__restrict a0, char *__restrict a1) {
  return ctime_r(a0, a1);
}

int f239(int a0, int a1) { return daemon(a0, a1); }

double f240(time_t a0, time_t a1) { return difftime(a0, a1); }

int f241(int a0) { return dup(a0); }

int f242(int a0, int a1) { return dup2(a0, a1); }

int f243(int a0, int a1, int a2) { return dup3(a0, a1, a2); }

int f244(int a0) { return dysize(a0); }

int f245(const char *a0, int a1) { return eaccess(a0, a1); }

void f246() { return endusershell(); }

int f247(const char *a0, int a1) { return euidaccess(a0, a1); }

template <typename... Args>
int f248(const char *a0, const char *a1, Args... args) {
  return execl(a0, a1, args...);
}

template <typename... Args>
int f249(const char *a0, const char *a1, Args... args) {
  return execle(a0, a1, args...);
}

template <typename... Args>
int f250(const char *a0, const char *a1, Args... args) {
  return execlp(a0, a1, args...);
}

int f251(const char *a0, char *const a1[]) { return execv(a0, a1); }

int f252(const char *a0, char *const a1[], char *const a2[]) {
  return execve(a0, a1, a2);
}

int f253(int a0, const char *a1, char *const a2[], char *const a3[], int a4) {
  return execveat(a0, a1, a2, a3, a4);
}

int f254(const char *a0, char *const a1[]) { return execvp(a0, a1); }

int f255(const char *a0, char *const a1[], char *const a2[]) {
  return execvpe(a0, a1, a2);
}

int f256(int a0, const char *a1, int a2, int a3) {
  return faccessat(a0, a1, a2, a3);
}

int f257(int a0) { return fchdir(a0); }

int f258(int a0, unsigned int a1, unsigned int a2) {
  return fchown(a0, a1, a2);
}

int f259(int a0, const char *a1, unsigned int a2, unsigned int a3, int a4) {
  return fchownat(a0, a1, a2, a3, a4);
}

int f260(int a0) { return fdatasync(a0); }

int f261(int a0, char *const a1[], char *const a2[]) {
  return fexecve(a0, a1, a2);
}

int f262() { return fork(); }

long f263(int a0, int a1) { return fpathconf(a0, a1); }

int f264(int a0) { return fsync(a0); }

int f265(int a0, long a1) { return ftruncate(a0, a1); }

int f266(int a0, long a1) { return ftruncate64(a0, a1); }

char *f267() { return get_current_dir_name(); }

int f268(unsigned int *a0, unsigned int *a1) { return getcpu(a0, a1); }

char *f269(char *a0, size_t a1) { return getcwd(a0, a1); }

struct tm *f270(const char *a0) { return getdate(a0); }

int f271(const char *__restrict a0, struct tm *__restrict a1) {
  return getdate_r(a0, a1);
}

int f272(char *a0, size_t a1) { return getdomainname(a0, a1); }

int f273() { return getdtablesize(); }

unsigned int f274() { return getegid(); }

int f275(void *a0, size_t a1) { return getentropy(a0, a1); }

unsigned int f276() { return geteuid(); }

unsigned int f277() { return getgid(); }

long f278() { return gethostid(); }

int f279(char *a0, size_t a1) { return gethostname(a0, a1); }

char *f280() { return getlogin(); }

int f281(char *a0, size_t a1) { return getlogin_r(a0, a1); }

int f282(int a0, char *const *a1, const char *a2) { return getopt(a0, a1, a2); }

int f283() { return getpagesize(); }

char *f284(const char *a0) { return getpass(a0); }

int f285(int a0) { return getpgid(a0); }

int f286() { return getpgrp(); }

int f287() { return getpid(); }

int f288() { return getppid(); }

int f289(unsigned int *a0, unsigned int *a1, unsigned int *a2) {
  return getresgid(a0, a1, a2);
}

int f290(unsigned int *a0, unsigned int *a1, unsigned int *a2) {
  return getresuid(a0, a1, a2);
}

int f291(int a0) { return getsid(a0); }

int f292() { return gettid(); }

unsigned int f293() { return getuid(); }

char *f294() { return getusershell(); }

struct tm *f295(const time_t *a0) { return gmtime(a0); }

struct tm *f296(const time_t *__restrict a0, struct tm *__restrict a1) {
  return gmtime_r(a0, a1);
}

int f297(unsigned int a0) { return group_member(a0); }

int f298(int a0) { return isatty(a0); }

using t68 = struct itimerspec;

int f299(const char *a0, unsigned int a1, unsigned int a2) {
  return lchown(a0, a1, a2);
}

int f300(const char *a0, const char *a1) { return link(a0, a1); }

int f301(int a0, const char *a1, int a2, const char *a3, int a4) {
  return linkat(a0, a1, a2, a3, a4);
}

struct tm *f302(const time_t *a0) { return localtime(a0); }

struct tm *f303(const time_t *__restrict a0, struct tm *__restrict a1) {
  return localtime_r(a0, a1);
}

int f304(int a0, int a1, long a2) { return lockf(a0, a1, a2); }

int f305(int a0, int a1, long a2) { return lockf64(a0, a1, a2); }

long f306(int a0, long a1, int a2) { return lseek(a0, a1, a2); }

long f307(int a0, long a1, int a2) { return lseek64(a0, a1, a2); }

time_t f308(struct tm *a0) { return mktime(a0); }

int f309(const struct timespec *a0, struct timespec *a1) {
  return nanosleep(a0, a1);
}

int f310(int a0) { return nice(a0); }

long f311(const char *a0, int a1) { return pathconf(a0, a1); }

int f312() { return pause(); }

int f313(int a0[2]) { return pipe(a0); }

int f314(int a0[2], int a1) { return pipe2(a0, a1); }

ssize_t f315(int a0, void *a1, size_t a2, long a3) {
  return pread(a0, a1, a2, a3);
}

ssize_t f316(int a0, void *a1, size_t a2, long a3) {
  return pread64(a0, a1, a2, a3);
}

int f317(unsigned short *a0, size_t a1, size_t a2, unsigned int a3) {
  return profil(a0, a1, a2, a3);
}

int f318(void (*a0)(), void (*a1)(), void (*a2)()) {
  return pthread_atfork(a0, a1, a2);
}

int f319(pthread_attr_t *a0) { return pthread_attr_destroy(a0); }

int f320(const pthread_attr_t *a0, size_t a1, cpu_set_t *a2) {
  return pthread_attr_getaffinity_np(a0, a1, a2);
}

int f321(const pthread_attr_t *a0, int *a1) {
  return pthread_attr_getdetachstate(a0, a1);
}

int f322(const pthread_attr_t *a0, size_t *a1) {
  return pthread_attr_getguardsize(a0, a1);
}

int f323(const pthread_attr_t *__restrict a0, int *__restrict a1) {
  return pthread_attr_getinheritsched(a0, a1);
}

int f324(const pthread_attr_t *__restrict a0,
         struct sched_param *__restrict a1) {
  return pthread_attr_getschedparam(a0, a1);
}

int f325(const pthread_attr_t *__restrict a0, int *__restrict a1) {
  return pthread_attr_getschedpolicy(a0, a1);
}

int f326(const pthread_attr_t *__restrict a0, int *__restrict a1) {
  return pthread_attr_getscope(a0, a1);
}

int f327(const pthread_attr_t *__restrict a0, void **__restrict a1,
         size_t *__restrict a2) {
  return pthread_attr_getstack(a0, a1, a2);
}

int f328(const pthread_attr_t *__restrict a0, size_t *__restrict a1) {
  return pthread_attr_getstacksize(a0, a1);
}

int f329(pthread_attr_t *a0) { return pthread_attr_init(a0); }

int f330(pthread_attr_t *a0, size_t a1, const cpu_set_t *a2) {
  return pthread_attr_setaffinity_np(a0, a1, a2);
}

int f331(pthread_attr_t *a0, int a1) {
  return pthread_attr_setdetachstate(a0, a1);
}

int f332(pthread_attr_t *a0, size_t a1) {
  return pthread_attr_setguardsize(a0, a1);
}

int f333(pthread_attr_t *a0, int a1) {
  return pthread_attr_setinheritsched(a0, a1);
}

int f334(pthread_attr_t *__restrict a0,
         const struct sched_param *__restrict a1) {
  return pthread_attr_setschedparam(a0, a1);
}

int f335(pthread_attr_t *a0, int a1) {
  return pthread_attr_setschedpolicy(a0, a1);
}

int f336(pthread_attr_t *a0, int a1) { return pthread_attr_setscope(a0, a1); }

int f337(pthread_attr_t *a0, void *a1, size_t a2) {
  return pthread_attr_setstack(a0, a1, a2);
}

int f338(pthread_attr_t *a0, size_t a1) {
  return pthread_attr_setstacksize(a0, a1);
}

int f339(pthread_barrier_t *a0) { return pthread_barrier_destroy(a0); }

int f340(pthread_barrier_t *__restrict a0,
         const pthread_barrierattr_t *__restrict a1, unsigned int a2) {
  return pthread_barrier_init(a0, a1, a2);
}

int f341(pthread_barrier_t *a0) { return pthread_barrier_wait(a0); }

int f342(pthread_barrierattr_t *a0) { return pthread_barrierattr_destroy(a0); }

int f343(const pthread_barrierattr_t *__restrict a0, int *__restrict a1) {
  return pthread_barrierattr_getpshared(a0, a1);
}

int f344(pthread_barrierattr_t *a0) { return pthread_barrierattr_init(a0); }

int f345(pthread_barrierattr_t *a0, int a1) {
  return pthread_barrierattr_setpshared(a0, a1);
}

int f346(pthread_t a0) { return pthread_cancel(a0); }

int f347(pthread_t a0, void **a1, clockid_t a2, const struct timespec *a3) {
  return pthread_clockjoin_np(a0, a1, a2, a3);
}

int f348(pthread_cond_t *a0) { return pthread_cond_broadcast(a0); }

int f349(pthread_cond_t *__restrict a0, pthread_mutex_t *__restrict a1, int a2,
         const struct timespec *__restrict a3) {
  return pthread_cond_clockwait(a0, a1, a2, a3);
}

int f350(pthread_cond_t *a0) { return pthread_cond_destroy(a0); }

int f351(pthread_cond_t *__restrict a0,
         const pthread_condattr_t *__restrict a1) {
  return pthread_cond_init(a0, a1);
}

int f352(pthread_cond_t *a0) { return pthread_cond_signal(a0); }

int f353(pthread_cond_t *__restrict a0, pthread_mutex_t *__restrict a1,
         const struct timespec *__restrict a2) {
  return pthread_cond_timedwait(a0, a1, a2);
}

int f354(pthread_cond_t *__restrict a0, pthread_mutex_t *__restrict a1) {
  return pthread_cond_wait(a0, a1);
}

int f355(pthread_condattr_t *a0) { return pthread_condattr_destroy(a0); }

int f356(const pthread_condattr_t *__restrict a0, int *__restrict a1) {
  return pthread_condattr_getclock(a0, a1);
}

int f357(const pthread_condattr_t *__restrict a0, int *__restrict a1) {
  return pthread_condattr_getpshared(a0, a1);
}

int f358(pthread_condattr_t *a0) { return pthread_condattr_init(a0); }

int f359(pthread_condattr_t *a0, int a1) {
  return pthread_condattr_setclock(a0, a1);
}

int f360(pthread_condattr_t *a0, int a1) {
  return pthread_condattr_setpshared(a0, a1);
}

int f361(pthread_t *__restrict a0, const pthread_attr_t *__restrict a1,
         void *(*a2)(void *), void *__restrict a3) {
  return pthread_create(a0, a1, a2, a3);
}

int f362(pthread_t a0) { return pthread_detach(a0); }

int f363(pthread_t a0, pthread_t a1) { return pthread_equal(a0, a1); }

void f364(void *a0) { return pthread_exit(a0); }

int f365(pthread_t a0, size_t a1, cpu_set_t *a2) {
  return pthread_getaffinity_np(a0, a1, a2);
}

int f366(pthread_attr_t *a0) { return pthread_getattr_default_np(a0); }

int f367(pthread_t a0, pthread_attr_t *a1) {
  return pthread_getattr_np(a0, a1);
}

int f368() { return pthread_getconcurrency(); }

int f369(pthread_t a0, int *a1) { return pthread_getcpuclockid(a0, a1); }

int f370(pthread_t a0, char *a1, size_t a2) {
  return pthread_getname_np(a0, a1, a2);
}

int f371(pthread_t a0, int *__restrict a1, struct sched_param *__restrict a2) {
  return pthread_getschedparam(a0, a1, a2);
}

void *f372(pthread_key_t a0) { return pthread_getspecific(a0); }

int f373(pthread_t a0, void **a1) { return pthread_join(a0, a1); }

int f374(pthread_key_t *a0, void (*a1)(void *)) {
  return pthread_key_create(a0, a1);
}

int f375(pthread_key_t a0) { return pthread_key_delete(a0); }

int f376(pthread_mutex_t *__restrict a0, clockid_t a1,
         const struct timespec *__restrict a2) {
  return pthread_mutex_clocklock(a0, a1, a2);
}

int f377(pthread_mutex_t *a0) { return pthread_mutex_consistent(a0); }

int f378(pthread_mutex_t *a0) { return pthread_mutex_destroy(a0); }

int f379(const pthread_mutex_t *__restrict a0, int *__restrict a1) {
  return pthread_mutex_getprioceiling(a0, a1);
}

int f380(pthread_mutex_t *a0, const pthread_mutexattr_t *a1) {
  return pthread_mutex_init(a0, a1);
}

int f381(pthread_mutex_t *a0) { return pthread_mutex_lock(a0); }

int f382(pthread_mutex_t *__restrict a0, int a1, int *__restrict a2) {
  return pthread_mutex_setprioceiling(a0, a1, a2);
}

int f383(pthread_mutex_t *__restrict a0, const struct timespec *__restrict a1) {
  return pthread_mutex_timedlock(a0, a1);
}

int f384(pthread_mutex_t *a0) { return pthread_mutex_trylock(a0); }

int f385(pthread_mutex_t *a0) { return pthread_mutex_unlock(a0); }

int f386(pthread_mutexattr_t *a0) { return pthread_mutexattr_destroy(a0); }

int f387(const pthread_mutexattr_t *__restrict a0, int *__restrict a1) {
  return pthread_mutexattr_getprioceiling(a0, a1);
}

int f388(const pthread_mutexattr_t *__restrict a0, int *__restrict a1) {
  return pthread_mutexattr_getprotocol(a0, a1);
}

int f389(const pthread_mutexattr_t *__restrict a0, int *__restrict a1) {
  return pthread_mutexattr_getpshared(a0, a1);
}

int f390(const pthread_mutexattr_t *a0, int *a1) {
  return pthread_mutexattr_getrobust(a0, a1);
}

int f391(const pthread_mutexattr_t *__restrict a0, int *__restrict a1) {
  return pthread_mutexattr_gettype(a0, a1);
}

int f392(pthread_mutexattr_t *a0) { return pthread_mutexattr_init(a0); }

int f393(pthread_mutexattr_t *a0, int a1) {
  return pthread_mutexattr_setprioceiling(a0, a1);
}

int f394(pthread_mutexattr_t *a0, int a1) {
  return pthread_mutexattr_setprotocol(a0, a1);
}

int f395(pthread_mutexattr_t *a0, int a1) {
  return pthread_mutexattr_setpshared(a0, a1);
}

int f396(pthread_mutexattr_t *a0, int a1) {
  return pthread_mutexattr_setrobust(a0, a1);
}

int f397(pthread_mutexattr_t *a0, int a1) {
  return pthread_mutexattr_settype(a0, a1);
}

int f398(pthread_once_t *a0, void (*a1)()) { return pthread_once(a0, a1); }

int f399(pthread_rwlock_t *__restrict a0, clockid_t a1,
         const struct timespec *__restrict a2) {
  return pthread_rwlock_clockrdlock(a0, a1, a2);
}

int f400(pthread_rwlock_t *__restrict a0, clockid_t a1,
         const struct timespec *__restrict a2) {
  return pthread_rwlock_clockwrlock(a0, a1, a2);
}

int f401(pthread_rwlock_t *a0) { return pthread_rwlock_destroy(a0); }

int f402(pthread_rwlock_t *__restrict a0,
         const pthread_rwlockattr_t *__restrict a1) {
  return pthread_rwlock_init(a0, a1);
}

int f403(pthread_rwlock_t *a0) { return pthread_rwlock_rdlock(a0); }

int f404(pthread_rwlock_t *__restrict a0,
         const struct timespec *__restrict a1) {
  return pthread_rwlock_timedrdlock(a0, a1);
}

int f405(pthread_rwlock_t *__restrict a0,
         const struct timespec *__restrict a1) {
  return pthread_rwlock_timedwrlock(a0, a1);
}

int f406(pthread_rwlock_t *a0) { return pthread_rwlock_tryrdlock(a0); }

int f407(pthread_rwlock_t *a0) { return pthread_rwlock_trywrlock(a0); }

int f408(pthread_rwlock_t *a0) { return pthread_rwlock_unlock(a0); }

int f409(pthread_rwlock_t *a0) { return pthread_rwlock_wrlock(a0); }

int f410(pthread_rwlockattr_t *a0) { return pthread_rwlockattr_destroy(a0); }

int f411(const pthread_rwlockattr_t *__restrict a0, int *__restrict a1) {
  return pthread_rwlockattr_getkind_np(a0, a1);
}

int f412(const pthread_rwlockattr_t *__restrict a0, int *__restrict a1) {
  return pthread_rwlockattr_getpshared(a0, a1);
}

int f413(pthread_rwlockattr_t *a0) { return pthread_rwlockattr_init(a0); }

int f414(pthread_rwlockattr_t *a0, int a1) {
  return pthread_rwlockattr_setkind_np(a0, a1);
}

int f415(pthread_rwlockattr_t *a0, int a1) {
  return pthread_rwlockattr_setpshared(a0, a1);
}

pthread_t f416() { return pthread_self(); }

int f417(pthread_t a0, size_t a1, const cpu_set_t *a2) {
  return pthread_setaffinity_np(a0, a1, a2);
}

int f418(const pthread_attr_t *a0) { return pthread_setattr_default_np(a0); }

int f419(int a0, int *a1) { return pthread_setcancelstate(a0, a1); }

int f420(int a0, int *a1) { return pthread_setcanceltype(a0, a1); }

int f421(int a0) { return pthread_setconcurrency(a0); }

int f422(pthread_t a0, const char *a1) { return pthread_setname_np(a0, a1); }

int f423(pthread_t a0, int a1, const struct sched_param *a2) {
  return pthread_setschedparam(a0, a1, a2);
}

int f424(pthread_t a0, int a1) { return pthread_setschedprio(a0, a1); }

int f425(pthread_key_t a0, const void *a1) {
  return pthread_setspecific(a0, a1);
}

int f426(pthread_spinlock_t *a0) { return pthread_spin_destroy(a0); }

int f427(pthread_spinlock_t *a0, int a1) { return pthread_spin_init(a0, a1); }

int f428(pthread_spinlock_t *a0) { return pthread_spin_lock(a0); }

int f429(pthread_spinlock_t *a0) { return pthread_spin_trylock(a0); }

int f430(pthread_spinlock_t *a0) { return pthread_spin_unlock(a0); }

void f431() { return pthread_testcancel(); }

int f432(pthread_t a0, void **a1, const struct timespec *a2) {
  return pthread_timedjoin_np(a0, a1, a2);
}

int f433(pthread_t a0, void **a1) { return pthread_tryjoin_np(a0, a1); }

ssize_t f434(int a0, const void *a1, size_t a2, long a3) {
  return pwrite(a0, a1, a2, a3);
}

ssize_t f435(int a0, const void *a1, size_t a2, long a3) {
  return pwrite64(a0, a1, a2, a3);
}

ssize_t f436(int a0, void *a1, size_t a2) { return read(a0, a1, a2); }

ssize_t f437(const char *__restrict a0, char *__restrict a1, size_t a2) {
  return readlink(a0, a1, a2);
}

ssize_t f438(int a0, const char *__restrict a1, char *__restrict a2,
             size_t a3) {
  return readlinkat(a0, a1, a2, a3);
}

int f439(const char *a0) { return revoke(a0); }

int f440(const char *a0) { return rmdir(a0); }

void *f441(intptr_t a0) { return sbrk(a0); }

int f442(int a0) { return sched_get_priority_max(a0); }

int f443(int a0) { return sched_get_priority_min(a0); }

int f444(int a0, size_t a1, cpu_set_t *a2) {
  return sched_getaffinity(a0, a1, a2);
}

int f445() { return sched_getcpu(); }

int f446(int a0, struct sched_param *a1) { return sched_getparam(a0, a1); }

int f447(int a0) { return sched_getscheduler(a0); }

using t69 = struct sched_param;

int f448(int a0, struct timespec *a1) { return sched_rr_get_interval(a0, a1); }

int f449(int a0, size_t a1, const cpu_set_t *a2) {
  return sched_setaffinity(a0, a1, a2);
}

int f450(int a0, const struct sched_param *a1) {
  return sched_setparam(a0, a1);
}

int f451(int a0, int a1, const struct sched_param *a2) {
  return sched_setscheduler(a0, a1, a2);
}

int f452() { return sched_yield(); }

int f453(const char *a0, size_t a1) { return setdomainname(a0, a1); }

int f454(unsigned int a0) { return setegid(a0); }

int f455(unsigned int a0) { return seteuid(a0); }

int f456(unsigned int a0) { return setgid(a0); }

int f457(long a0) { return sethostid(a0); }

int f458(const char *a0, size_t a1) { return sethostname(a0, a1); }

int f459(const char *a0) { return setlogin(a0); }

int f460(int a0, int a1) { return setns(a0, a1); }

int f461(int a0, int a1) { return setpgid(a0, a1); }

int f462() { return setpgrp(); }

int f463(unsigned int a0, unsigned int a1) { return setregid(a0, a1); }

int f464(unsigned int a0, unsigned int a1, unsigned int a2) {
  return setresgid(a0, a1, a2);
}

int f465(unsigned int a0, unsigned int a1, unsigned int a2) {
  return setresuid(a0, a1, a2);
}

int f466(unsigned int a0, unsigned int a1) { return setreuid(a0, a1); }

int f467() { return setsid(); }

int f468(unsigned int a0) { return setuid(a0); }

void f469() { return setusershell(); }

unsigned int f470(unsigned int a0) { return sleep(a0); }

using t70 = socklen_t;

void f471(const std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f472(std::atomic<char> &a0) { return a0.notify_one(); }

void f473(std::atomic<char> &a0) { return a0.notify_all(); }

using t71 = std::atomic<long double>;

std::atomic<long double> f474() { return {}; }

std::atomic<long double> f475(long double a0) { return {a0}; }

bool f476(const std::atomic<long double> &a0) { return a0.is_lock_free(); }

void f477(std::atomic<long double> &a0, long double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

long double f478(const std::atomic<long double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

long double f479(const std::atomic<long double> &a0) {
  return static_cast<long double>(a0);
}

long double f480(std::atomic<long double> &a0, long double a1,
                 std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f481(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f482(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f483(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f484(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f485(const std::atomic<long double> &a0, long double a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f486(const std::atomic<long double> &a0) { return a0.notify_one(); }

void f487(const std::atomic<long double> &a0) { return a0.notify_all(); }

void f488(const std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f489(std::atomic<int> &a0) { return a0.notify_one(); }

void f490(std::atomic<int> &a0) { return a0.notify_all(); }

void f491(const std::atomic<long long> &a0, long long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f492(std::atomic<long long> &a0) { return a0.notify_one(); }

void f493(std::atomic<long long> &a0) { return a0.notify_all(); }

void f494(const std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f495(std::atomic<long> &a0) { return a0.notify_one(); }

void f496(std::atomic<long> &a0) { return a0.notify_all(); }

void f497(const std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f498(std::atomic<short> &a0) { return a0.notify_one(); }

void f499(std::atomic<short> &a0) { return a0.notify_all(); }

void f500(const std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f501(std::atomic<wchar_t> &a0) { return a0.notify_one(); }

void f502(std::atomic<wchar_t> &a0) { return a0.notify_all(); }

void f503(const std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f504(std::atomic<bool> &a0) { return a0.notify_one(); }

void f505(std::atomic<bool> &a0) { return a0.notify_all(); }

void f506(const std::atomic<unsigned char> &a0, unsigned char a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f507(std::atomic<unsigned char> &a0) { return a0.notify_one(); }

void f508(std::atomic<unsigned char> &a0) { return a0.notify_all(); }

using t72 = std::atomic<double>;

std::atomic<double> f509() { return {}; }

std::atomic<double> f510(double a0) { return {a0}; }

bool f511(const std::atomic<double> &a0) { return a0.is_lock_free(); }

void f512(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

double f513(const std::atomic<double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

double f514(const std::atomic<double> &a0) { return static_cast<double>(a0); }

double f515(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f516(std::atomic<double> &a0, double &a1, double a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f517(std::atomic<double> &a0, double &a1, double a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f518(std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f519(std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f520(const std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f521(const std::atomic<double> &a0) { return a0.notify_one(); }

void f522(const std::atomic<double> &a0) { return a0.notify_all(); }

using t73 = std::atomic<float>;

std::atomic<float> f523() { return {}; }

std::atomic<float> f524(float a0) { return {a0}; }

bool f525(const std::atomic<float> &a0) { return a0.is_lock_free(); }

void f526(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

float f527(const std::atomic<float> &a0, std::memory_order a1) {
  return a0.load(a1);
}

float f528(const std::atomic<float> &a0) { return static_cast<float>(a0); }

float f529(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f530(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f531(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f532(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f533(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f534(const std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f535(const std::atomic<float> &a0) { return a0.notify_one(); }

void f536(const std::atomic<float> &a0) { return a0.notify_all(); }

void f537(const std::atomic<unsigned int> &a0, unsigned int a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f538(std::atomic<unsigned int> &a0) { return a0.notify_one(); }

void f539(std::atomic<unsigned int> &a0) { return a0.notify_all(); }

void f540(const std::atomic<unsigned long long> &a0, unsigned long long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f541(std::atomic<unsigned long long> &a0) { return a0.notify_one(); }

void f542(std::atomic<unsigned long long> &a0) { return a0.notify_all(); }

void f543(const std::atomic<unsigned long> &a0, unsigned long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f544(std::atomic<unsigned long> &a0) { return a0.notify_one(); }

void f545(std::atomic<unsigned long> &a0) { return a0.notify_all(); }

void f546(const std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f547(std::atomic<char16_t> &a0) { return a0.notify_one(); }

void f548(std::atomic<char16_t> &a0) { return a0.notify_all(); }

void f549(const std::atomic<signed char> &a0, signed char a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f550(std::atomic<signed char> &a0) { return a0.notify_one(); }

void f551(std::atomic<signed char> &a0) { return a0.notify_all(); }

void f552(const std::atomic<unsigned short> &a0, unsigned short a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f553(std::atomic<unsigned short> &a0) { return a0.notify_one(); }

void f554(std::atomic<unsigned short> &a0) { return a0.notify_all(); }

using t74 = std::atomic<char8_t>;

std::atomic<char8_t> f555() { return {}; }

std::atomic<char8_t> f556(char8_t a0) { return {a0}; }

bool f557(const std::atomic<char8_t> &a0) { return a0.is_lock_free(); }

void f558(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f559(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f560(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f561(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f562(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f563(const std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f564(std::atomic<char8_t> &a0) { return a0.notify_one(); }

void f565(std::atomic<char8_t> &a0) { return a0.notify_all(); }

bool f566() { return std::atomic<char8_t>::is_always_lock_free; }

void f567(const std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f568(std::atomic<char32_t> &a0) { return a0.notify_one(); }

void f569(std::atomic<char32_t> &a0) { return a0.notify_all(); }

template <typename T1>
void f570(const std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f571(std::atomic<T1 *> &a0) {
  return a0.notify_one();
}

template <typename T1> void f572(std::atomic<T1 *> &a0) {
  return a0.notify_all();
}

template <typename T1> using t75 = std::atomic<std::shared_ptr<T1>>;

template <typename T1>
using t76 = typename std::atomic<std::shared_ptr<T1>>::value_type;

template <typename T1> bool f573(const std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.is_lock_free();
}

template <typename T1> std::atomic<std::shared_ptr<T1>> f574() { return {}; }

template <typename T1>
std::atomic<std::shared_ptr<T1>> f575(std::nullptr_t a0) {
  return {a0};
}

template <typename T1>
std::atomic<std::shared_ptr<T1>> f576(std::shared_ptr<T1> a0) {
  return {a0};
}

template <typename T1>
std::shared_ptr<T1> f577(const std::atomic<std::shared_ptr<T1>> &a0,
                         std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
std::shared_ptr<T1> f578(const std::atomic<std::shared_ptr<T1>> &a0) {
  return static_cast<std::shared_ptr<T1>>(a0);
}

template <typename T1>
void f579(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
void f580(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1>
void f581(std::atomic<std::shared_ptr<T1>> &a0, std::nullptr_t a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::shared_ptr<T1> f582(std::atomic<std::shared_ptr<T1>> &a0,
                         std::shared_ptr<T1> a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f583(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f584(std::atomic<std::shared_ptr<T1>> &a0,
          typename std::atomic<std::shared_ptr<T1>>::value_type &a1,
          typename std::atomic<std::shared_ptr<T1>>::value_type a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
bool f585(std::atomic<std::shared_ptr<T1>> &a0,
          typename std::atomic<std::shared_ptr<T1>>::value_type &a1,
          typename std::atomic<std::shared_ptr<T1>>::value_type a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f586(std::atomic<std::shared_ptr<T1>> &a0,
          typename std::atomic<std::shared_ptr<T1>>::value_type &a1,
          typename std::atomic<std::shared_ptr<T1>>::value_type a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
void f587(const std::atomic<std::shared_ptr<T1>> &a0,
          typename std::atomic<std::shared_ptr<T1>>::value_type a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f588(std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.notify_one();
}

template <typename T1> void f589(std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.notify_all();
}

template <typename T1>
using t77 = typename std::atomic<std::shared_ptr<T1>>::value_type::element_type;

template <typename T1>
using t78 = typename std::atomic<std::shared_ptr<T1>>::value_type::weak_type;

template <typename T1> bool f590() {
  return std::atomic<std::shared_ptr<T1>>::is_always_lock_free;
}

template <typename T1> using t79 = std::atomic<std::weak_ptr<T1>>;

template <typename T1>
using t80 = typename std::atomic<std::weak_ptr<T1>>::value_type;

template <typename T1> bool f591(const std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.is_lock_free();
}

template <typename T1> std::atomic<std::weak_ptr<T1>> f592() { return {}; }

template <typename T1>
std::atomic<std::weak_ptr<T1>> f593(std::weak_ptr<T1> a0) {
  return {a0};
}

template <typename T1>
std::weak_ptr<T1> f594(const std::atomic<std::weak_ptr<T1>> &a0,
                       std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
std::weak_ptr<T1> f595(const std::atomic<std::weak_ptr<T1>> &a0) {
  return static_cast<std::weak_ptr<T1>>(a0);
}

template <typename T1>
void f596(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
void f597(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::weak_ptr<T1> f598(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
                       std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f599(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
          std::weak_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f600(std::atomic<std::weak_ptr<T1>> &a0,
          typename std::atomic<std::weak_ptr<T1>>::value_type &a1,
          typename std::atomic<std::weak_ptr<T1>>::value_type a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
bool f601(std::atomic<std::weak_ptr<T1>> &a0,
          typename std::atomic<std::weak_ptr<T1>>::value_type &a1,
          typename std::atomic<std::weak_ptr<T1>>::value_type a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f602(std::atomic<std::weak_ptr<T1>> &a0,
          typename std::atomic<std::weak_ptr<T1>>::value_type &a1,
          typename std::atomic<std::weak_ptr<T1>>::value_type a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
void f603(const std::atomic<std::weak_ptr<T1>> &a0,
          typename std::atomic<std::weak_ptr<T1>>::value_type a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f604(std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.notify_one();
}

template <typename T1> void f605(std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.notify_all();
}

template <typename T1>
std::shared_ptr<T1>
f606(const typename std::atomic<std::weak_ptr<T1>>::value_type &a0) {
  return a0.lock();
}

template <typename T1> bool f607() {
  return std::atomic<std::weak_ptr<T1>>::is_always_lock_free;
}

template <typename T1>
void f608(const std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f609(std::atomic<T1> &a0) {
  return a0.notify_one();
}

template <typename T1> void f610(std::atomic<T1> &a0) {
  return a0.notify_all();
}

using t81 = std::atomic_char8_t;

void f611(std::atomic_flag *a0) { return std::atomic_flag_notify_all(a0); }

void f612(std::atomic_flag *a0) { return std::atomic_flag_notify_one(a0); }

bool f613(const std::atomic_flag *a0) { return std::atomic_flag_test(a0); }

bool f614(const volatile std::atomic_flag *a0) {
  return std::atomic_flag_test(a0);
}

bool f615(const std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}

bool f616(const volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}

void f617(std::atomic_flag *a0, bool a1) {
  return std::atomic_flag_wait(a0, a1);
}

void f618(std::atomic_flag *a0, bool a1, std::memory_order a2) {
  return std::atomic_flag_wait_explicit(a0, a1, a2);
}

template <typename T1> void f619(std::atomic<T1> *a0) {
  return std::atomic_notify_all<T1>(a0);
}

template <typename T1> void f620(std::atomic<T1> *a0) {
  return std::atomic_notify_one<T1>(a0);
}

using t82 = std::atomic_signed_lock_free;

using t83 = std::atomic_unsigned_lock_free;

template <typename T1>
void f621(const std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_wait<T1>(a0, a1);
}

template <typename T1>
void f622(const std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
          std::memory_order a2) {
  return std::atomic_wait_explicit<T1>(a0, a1, a2);
}

size_t f623(char *__restrict a0, size_t a1, const char *__restrict a2,
            const struct tm *__restrict a3) {
  return strftime(a0, a1, a2, a3);
}

size_t f624(char *__restrict a0, size_t a1, const char *__restrict a2,
            const struct tm *__restrict a3, locale_t a4) {
  return strftime_l(a0, a1, a2, a3, a4);
}

char *f625(const char *__restrict a0, const char *__restrict a1,
           struct tm *a2) {
  return strptime(a0, a1, a2);
}

char *f626(const char *__restrict a0, const char *__restrict a1, struct tm *a2,
           locale_t a3) {
  return strptime_l(a0, a1, a2, a3);
}

void f627(const void *__restrict a0, void *__restrict a1, ssize_t a2) {
  return swab(a0, a1, a2);
}

int f628(const char *a0, const char *a1) { return symlink(a0, a1); }

int f629(const char *a0, int a1, const char *a2) {
  return symlinkat(a0, a1, a2);
}

void f630() { return sync(); }

int f631(int a0) { return syncfs(a0); }

template <typename... Args> long f632(long a0, Args... args) {
  return syscall(a0, args...);
}

long f633(int a0) { return sysconf(a0); }

int f634(int a0) { return tcgetpgrp(a0); }

int f635(int a0, int a1) { return tcsetpgrp(a0, a1); }

time_t f636(time_t *a0) { return time(a0); }

time_t f637(struct tm *a0) { return timegm(a0); }

time_t f638(struct tm *a0) { return timelocal(a0); }

int f639(clockid_t a0, struct sigevent *__restrict a1, timer_t *__restrict a2) {
  return timer_create(a0, a1, a2);
}

int f640(timer_t a0) { return timer_delete(a0); }

int f641(timer_t a0) { return timer_getoverrun(a0); }

int f642(timer_t a0, struct itimerspec *a1) { return timer_gettime(a0, a1); }

int f643(timer_t a0, int a1, const struct itimerspec *__restrict a2,
         struct itimerspec *__restrict a3) {
  return timer_settime(a0, a1, a2, a3);
}

int f644(struct timespec *a0, int a1) { return timespec_get(a0, a1); }

int f645(struct timespec *a0, int a1) { return timespec_getres(a0, a1); }

using t84 = struct timex;

using t85 = struct tm;

int f646(const char *a0, long a1) { return truncate(a0, a1); }

int f647(const char *a0, long a1) { return truncate64(a0, a1); }

char *f648(int a0) { return ttyname(a0); }

int f649(int a0, char *a1, size_t a2) { return ttyname_r(a0, a1, a2); }

int f650() { return ttyslot(); }

void f651() { return tzset(); }

unsigned int f652(unsigned int a0, unsigned int a1) { return ualarm(a0, a1); }

int f653(const char *a0) { return unlink(a0); }

int f654(int a0, const char *a1, int a2) { return unlinkat(a0, a1, a2); }

int f655(int a0) { return unshare(a0); }

int f656(unsigned int a0) { return usleep(a0); }

int f657() { return vfork(); }

int f658() { return vhangup(); }

ssize_t f659(int a0, const void *a1, size_t a2) { return write(a0, a1, a2); }
