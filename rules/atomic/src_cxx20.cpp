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

char *f217(const struct tm *__restrict a0, char *__restrict a1) {
  return asctime_r(a0, a1);
}

int f218(void *a0) { return brk(a0); }

int f219(const char *a0) { return chdir(a0); }

int f220(const char *a0, unsigned int a1, unsigned int a2) {
  return chown(a0, a1, a2);
}

int f221(const char *a0) { return chroot(a0); }

int f222(int a0, struct timex *a1) { return clock_adjtime(a0, a1); }

int f223(pid_t a0, clockid_t *a1) { return clock_getcpuclockid(a0, a1); }

int f224(clockid_t a0, struct timespec *a1) { return clock_getres(a0, a1); }

int f225(clockid_t a0, struct timespec *a1) { return clock_gettime(a0, a1); }

int f226(clockid_t a0, int a1, const struct timespec *a2, struct timespec *a3) {
  return clock_nanosleep(a0, a1, a2, a3);
}

int f227(clockid_t a0, const struct timespec *a1) {
  return clock_settime(a0, a1);
}

template <typename... Args>
int f228(int (*a0)(void *), void *a1, int a2, void *a3, Args... args) {
  return clone(a0, a1, a2, a3, args...);
}

int f229(int a0) { return close(a0); }

int f230(unsigned int a0, unsigned int a1, int a2) {
  return close_range(a0, a1, a2);
}

void f231(int a0) { return closefrom(a0); }

size_t f232(int a0, char *a1, size_t a2) { return confstr(a0, a1, a2); }

ssize_t f233(int a0, long *a1, int a2, long *a3, size_t a4, unsigned int a5) {
  return copy_file_range(a0, a1, a2, a3, a4, a5);
}

using t67 = cpu_set_t;

char *f234(const char *a0, const char *a1) { return crypt(a0, a1); }

char *f235(const time_t *__restrict a0, char *__restrict a1) {
  return ctime_r(a0, a1);
}

int f236(int a0, int a1) { return daemon(a0, a1); }

int f237(int a0) { return dup(a0); }

int f238(int a0, int a1) { return dup2(a0, a1); }

int f239(int a0, int a1, int a2) { return dup3(a0, a1, a2); }

int f240(int a0) { return dysize(a0); }

int f241(const char *a0, int a1) { return eaccess(a0, a1); }

void f242() { return endusershell(); }

int f243(const char *a0, int a1) { return euidaccess(a0, a1); }

template <typename... Args>
int f244(const char *a0, const char *a1, Args... args) {
  return execl(a0, a1, args...);
}

template <typename... Args>
int f245(const char *a0, const char *a1, Args... args) {
  return execle(a0, a1, args...);
}

template <typename... Args>
int f246(const char *a0, const char *a1, Args... args) {
  return execlp(a0, a1, args...);
}

int f247(const char *a0, char *const a1[]) { return execv(a0, a1); }

int f248(const char *a0, char *const a1[], char *const a2[]) {
  return execve(a0, a1, a2);
}

int f249(int a0, const char *a1, char *const a2[], char *const a3[], int a4) {
  return execveat(a0, a1, a2, a3, a4);
}

int f250(const char *a0, char *const a1[]) { return execvp(a0, a1); }

int f251(const char *a0, char *const a1[], char *const a2[]) {
  return execvpe(a0, a1, a2);
}

int f252(int a0, const char *a1, int a2, int a3) {
  return faccessat(a0, a1, a2, a3);
}

int f253(int a0) { return fchdir(a0); }

int f254(int a0, unsigned int a1, unsigned int a2) {
  return fchown(a0, a1, a2);
}

int f255(int a0, const char *a1, unsigned int a2, unsigned int a3, int a4) {
  return fchownat(a0, a1, a2, a3, a4);
}

int f256(int a0) { return fdatasync(a0); }

int f257(int a0, char *const a1[], char *const a2[]) {
  return fexecve(a0, a1, a2);
}

int f258() { return fork(); }

long f259(int a0, int a1) { return fpathconf(a0, a1); }

int f260(int a0) { return fsync(a0); }

int f261(int a0, long a1) { return ftruncate(a0, a1); }

int f262(int a0, long a1) { return ftruncate64(a0, a1); }

char *f263() { return get_current_dir_name(); }

int f264(unsigned int *a0, unsigned int *a1) { return getcpu(a0, a1); }

char *f265(char *a0, size_t a1) { return getcwd(a0, a1); }

struct tm *f266(const char *a0) { return getdate(a0); }

int f267(const char *__restrict a0, struct tm *__restrict a1) {
  return getdate_r(a0, a1);
}

int f268(char *a0, size_t a1) { return getdomainname(a0, a1); }

int f269() { return getdtablesize(); }

unsigned int f270() { return getegid(); }

int f271(void *a0, size_t a1) { return getentropy(a0, a1); }

unsigned int f272() { return geteuid(); }

unsigned int f273() { return getgid(); }

long f274() { return gethostid(); }

int f275(char *a0, size_t a1) { return gethostname(a0, a1); }

char *f276() { return getlogin(); }

int f277(char *a0, size_t a1) { return getlogin_r(a0, a1); }

int f278(int a0, char *const *a1, const char *a2) { return getopt(a0, a1, a2); }

int f279() { return getpagesize(); }

char *f280(const char *a0) { return getpass(a0); }

int f281(int a0) { return getpgid(a0); }

int f282() { return getpgrp(); }

int f283() { return getpid(); }

int f284() { return getppid(); }

int f285(unsigned int *a0, unsigned int *a1, unsigned int *a2) {
  return getresgid(a0, a1, a2);
}

int f286(unsigned int *a0, unsigned int *a1, unsigned int *a2) {
  return getresuid(a0, a1, a2);
}

int f287(int a0) { return getsid(a0); }

int f288() { return gettid(); }

unsigned int f289() { return getuid(); }

char *f290() { return getusershell(); }

struct tm *f291(const time_t *__restrict a0, struct tm *__restrict a1) {
  return gmtime_r(a0, a1);
}

int f292(unsigned int a0) { return group_member(a0); }

int f293(int a0) { return isatty(a0); }

using t68 = struct itimerspec;

int f294(const char *a0, unsigned int a1, unsigned int a2) {
  return lchown(a0, a1, a2);
}

int f295(const char *a0, const char *a1) { return link(a0, a1); }

int f296(int a0, const char *a1, int a2, const char *a3, int a4) {
  return linkat(a0, a1, a2, a3, a4);
}

struct tm *f297(const time_t *__restrict a0, struct tm *__restrict a1) {
  return localtime_r(a0, a1);
}

int f298(int a0, int a1, long a2) { return lockf(a0, a1, a2); }

int f299(int a0, int a1, long a2) { return lockf64(a0, a1, a2); }

long f300(int a0, long a1, int a2) { return lseek(a0, a1, a2); }

long f301(int a0, long a1, int a2) { return lseek64(a0, a1, a2); }

int f302(const struct timespec *a0, struct timespec *a1) {
  return nanosleep(a0, a1);
}

int f303(int a0) { return nice(a0); }

long f304(const char *a0, int a1) { return pathconf(a0, a1); }

int f305() { return pause(); }

int f306(int a0[2]) { return pipe(a0); }

int f307(int a0[2], int a1) { return pipe2(a0, a1); }

ssize_t f308(int a0, void *a1, size_t a2, long a3) {
  return pread(a0, a1, a2, a3);
}

ssize_t f309(int a0, void *a1, size_t a2, long a3) {
  return pread64(a0, a1, a2, a3);
}

int f310(unsigned short *a0, size_t a1, size_t a2, unsigned int a3) {
  return profil(a0, a1, a2, a3);
}

int f311(void (*a0)(), void (*a1)(), void (*a2)()) {
  return pthread_atfork(a0, a1, a2);
}

int f312(pthread_attr_t *a0) { return pthread_attr_destroy(a0); }

int f313(const pthread_attr_t *a0, size_t a1, cpu_set_t *a2) {
  return pthread_attr_getaffinity_np(a0, a1, a2);
}

int f314(const pthread_attr_t *a0, int *a1) {
  return pthread_attr_getdetachstate(a0, a1);
}

int f315(const pthread_attr_t *a0, size_t *a1) {
  return pthread_attr_getguardsize(a0, a1);
}

int f316(const pthread_attr_t *__restrict a0, int *__restrict a1) {
  return pthread_attr_getinheritsched(a0, a1);
}

int f317(const pthread_attr_t *__restrict a0,
         struct sched_param *__restrict a1) {
  return pthread_attr_getschedparam(a0, a1);
}

int f318(const pthread_attr_t *__restrict a0, int *__restrict a1) {
  return pthread_attr_getschedpolicy(a0, a1);
}

int f319(const pthread_attr_t *__restrict a0, int *__restrict a1) {
  return pthread_attr_getscope(a0, a1);
}

int f320(const pthread_attr_t *__restrict a0, void **__restrict a1,
         size_t *__restrict a2) {
  return pthread_attr_getstack(a0, a1, a2);
}

int f321(const pthread_attr_t *__restrict a0, size_t *__restrict a1) {
  return pthread_attr_getstacksize(a0, a1);
}

int f322(pthread_attr_t *a0) { return pthread_attr_init(a0); }

int f323(pthread_attr_t *a0, size_t a1, const cpu_set_t *a2) {
  return pthread_attr_setaffinity_np(a0, a1, a2);
}

int f324(pthread_attr_t *a0, int a1) {
  return pthread_attr_setdetachstate(a0, a1);
}

int f325(pthread_attr_t *a0, size_t a1) {
  return pthread_attr_setguardsize(a0, a1);
}

int f326(pthread_attr_t *a0, int a1) {
  return pthread_attr_setinheritsched(a0, a1);
}

int f327(pthread_attr_t *__restrict a0,
         const struct sched_param *__restrict a1) {
  return pthread_attr_setschedparam(a0, a1);
}

int f328(pthread_attr_t *a0, int a1) {
  return pthread_attr_setschedpolicy(a0, a1);
}

int f329(pthread_attr_t *a0, int a1) { return pthread_attr_setscope(a0, a1); }

int f330(pthread_attr_t *a0, void *a1, size_t a2) {
  return pthread_attr_setstack(a0, a1, a2);
}

int f331(pthread_attr_t *a0, size_t a1) {
  return pthread_attr_setstacksize(a0, a1);
}

int f332(pthread_barrier_t *a0) { return pthread_barrier_destroy(a0); }

int f333(pthread_barrier_t *__restrict a0,
         const pthread_barrierattr_t *__restrict a1, unsigned int a2) {
  return pthread_barrier_init(a0, a1, a2);
}

int f334(pthread_barrier_t *a0) { return pthread_barrier_wait(a0); }

int f335(pthread_barrierattr_t *a0) { return pthread_barrierattr_destroy(a0); }

int f336(const pthread_barrierattr_t *__restrict a0, int *__restrict a1) {
  return pthread_barrierattr_getpshared(a0, a1);
}

int f337(pthread_barrierattr_t *a0) { return pthread_barrierattr_init(a0); }

int f338(pthread_barrierattr_t *a0, int a1) {
  return pthread_barrierattr_setpshared(a0, a1);
}

int f339(pthread_t a0) { return pthread_cancel(a0); }

int f340(pthread_t a0, void **a1, clockid_t a2, const struct timespec *a3) {
  return pthread_clockjoin_np(a0, a1, a2, a3);
}

int f341(pthread_cond_t *a0) { return pthread_cond_broadcast(a0); }

int f342(pthread_cond_t *__restrict a0, pthread_mutex_t *__restrict a1, int a2,
         const struct timespec *__restrict a3) {
  return pthread_cond_clockwait(a0, a1, a2, a3);
}

int f343(pthread_cond_t *a0) { return pthread_cond_destroy(a0); }

int f344(pthread_cond_t *__restrict a0,
         const pthread_condattr_t *__restrict a1) {
  return pthread_cond_init(a0, a1);
}

int f345(pthread_cond_t *a0) { return pthread_cond_signal(a0); }

int f346(pthread_cond_t *__restrict a0, pthread_mutex_t *__restrict a1,
         const struct timespec *__restrict a2) {
  return pthread_cond_timedwait(a0, a1, a2);
}

int f347(pthread_cond_t *__restrict a0, pthread_mutex_t *__restrict a1) {
  return pthread_cond_wait(a0, a1);
}

int f348(pthread_condattr_t *a0) { return pthread_condattr_destroy(a0); }

int f349(const pthread_condattr_t *__restrict a0, int *__restrict a1) {
  return pthread_condattr_getclock(a0, a1);
}

int f350(const pthread_condattr_t *__restrict a0, int *__restrict a1) {
  return pthread_condattr_getpshared(a0, a1);
}

int f351(pthread_condattr_t *a0) { return pthread_condattr_init(a0); }

int f352(pthread_condattr_t *a0, int a1) {
  return pthread_condattr_setclock(a0, a1);
}

int f353(pthread_condattr_t *a0, int a1) {
  return pthread_condattr_setpshared(a0, a1);
}

int f354(pthread_t *__restrict a0, const pthread_attr_t *__restrict a1,
         void *(*a2)(void *), void *__restrict a3) {
  return pthread_create(a0, a1, a2, a3);
}

int f355(pthread_t a0) { return pthread_detach(a0); }

int f356(pthread_t a0, pthread_t a1) { return pthread_equal(a0, a1); }

void f357(void *a0) { return pthread_exit(a0); }

int f358(pthread_t a0, size_t a1, cpu_set_t *a2) {
  return pthread_getaffinity_np(a0, a1, a2);
}

int f359(pthread_attr_t *a0) { return pthread_getattr_default_np(a0); }

int f360(pthread_t a0, pthread_attr_t *a1) {
  return pthread_getattr_np(a0, a1);
}

int f361() { return pthread_getconcurrency(); }

int f362(pthread_t a0, int *a1) { return pthread_getcpuclockid(a0, a1); }

int f363(pthread_t a0, char *a1, size_t a2) {
  return pthread_getname_np(a0, a1, a2);
}

int f364(pthread_t a0, int *__restrict a1, struct sched_param *__restrict a2) {
  return pthread_getschedparam(a0, a1, a2);
}

void *f365(pthread_key_t a0) { return pthread_getspecific(a0); }

int f366(pthread_t a0, void **a1) { return pthread_join(a0, a1); }

int f367(pthread_key_t *a0, void (*a1)(void *)) {
  return pthread_key_create(a0, a1);
}

int f368(pthread_key_t a0) { return pthread_key_delete(a0); }

int f369(pthread_mutex_t *__restrict a0, clockid_t a1,
         const struct timespec *__restrict a2) {
  return pthread_mutex_clocklock(a0, a1, a2);
}

int f370(pthread_mutex_t *a0) { return pthread_mutex_consistent(a0); }

int f371(pthread_mutex_t *a0) { return pthread_mutex_destroy(a0); }

int f372(const pthread_mutex_t *__restrict a0, int *__restrict a1) {
  return pthread_mutex_getprioceiling(a0, a1);
}

int f373(pthread_mutex_t *a0, const pthread_mutexattr_t *a1) {
  return pthread_mutex_init(a0, a1);
}

int f374(pthread_mutex_t *a0) { return pthread_mutex_lock(a0); }

int f375(pthread_mutex_t *__restrict a0, int a1, int *__restrict a2) {
  return pthread_mutex_setprioceiling(a0, a1, a2);
}

int f376(pthread_mutex_t *__restrict a0, const struct timespec *__restrict a1) {
  return pthread_mutex_timedlock(a0, a1);
}

int f377(pthread_mutex_t *a0) { return pthread_mutex_trylock(a0); }

int f378(pthread_mutex_t *a0) { return pthread_mutex_unlock(a0); }

int f379(pthread_mutexattr_t *a0) { return pthread_mutexattr_destroy(a0); }

int f380(const pthread_mutexattr_t *__restrict a0, int *__restrict a1) {
  return pthread_mutexattr_getprioceiling(a0, a1);
}

int f381(const pthread_mutexattr_t *__restrict a0, int *__restrict a1) {
  return pthread_mutexattr_getprotocol(a0, a1);
}

int f382(const pthread_mutexattr_t *__restrict a0, int *__restrict a1) {
  return pthread_mutexattr_getpshared(a0, a1);
}

int f383(const pthread_mutexattr_t *a0, int *a1) {
  return pthread_mutexattr_getrobust(a0, a1);
}

int f384(const pthread_mutexattr_t *__restrict a0, int *__restrict a1) {
  return pthread_mutexattr_gettype(a0, a1);
}

int f385(pthread_mutexattr_t *a0) { return pthread_mutexattr_init(a0); }

int f386(pthread_mutexattr_t *a0, int a1) {
  return pthread_mutexattr_setprioceiling(a0, a1);
}

int f387(pthread_mutexattr_t *a0, int a1) {
  return pthread_mutexattr_setprotocol(a0, a1);
}

int f388(pthread_mutexattr_t *a0, int a1) {
  return pthread_mutexattr_setpshared(a0, a1);
}

int f389(pthread_mutexattr_t *a0, int a1) {
  return pthread_mutexattr_setrobust(a0, a1);
}

int f390(pthread_mutexattr_t *a0, int a1) {
  return pthread_mutexattr_settype(a0, a1);
}

int f391(pthread_once_t *a0, void (*a1)()) { return pthread_once(a0, a1); }

int f392(pthread_rwlock_t *__restrict a0, clockid_t a1,
         const struct timespec *__restrict a2) {
  return pthread_rwlock_clockrdlock(a0, a1, a2);
}

int f393(pthread_rwlock_t *__restrict a0, clockid_t a1,
         const struct timespec *__restrict a2) {
  return pthread_rwlock_clockwrlock(a0, a1, a2);
}

int f394(pthread_rwlock_t *a0) { return pthread_rwlock_destroy(a0); }

int f395(pthread_rwlock_t *__restrict a0,
         const pthread_rwlockattr_t *__restrict a1) {
  return pthread_rwlock_init(a0, a1);
}

int f396(pthread_rwlock_t *a0) { return pthread_rwlock_rdlock(a0); }

int f397(pthread_rwlock_t *__restrict a0,
         const struct timespec *__restrict a1) {
  return pthread_rwlock_timedrdlock(a0, a1);
}

int f398(pthread_rwlock_t *__restrict a0,
         const struct timespec *__restrict a1) {
  return pthread_rwlock_timedwrlock(a0, a1);
}

int f399(pthread_rwlock_t *a0) { return pthread_rwlock_tryrdlock(a0); }

int f400(pthread_rwlock_t *a0) { return pthread_rwlock_trywrlock(a0); }

int f401(pthread_rwlock_t *a0) { return pthread_rwlock_unlock(a0); }

int f402(pthread_rwlock_t *a0) { return pthread_rwlock_wrlock(a0); }

int f403(pthread_rwlockattr_t *a0) { return pthread_rwlockattr_destroy(a0); }

int f404(const pthread_rwlockattr_t *__restrict a0, int *__restrict a1) {
  return pthread_rwlockattr_getkind_np(a0, a1);
}

int f405(const pthread_rwlockattr_t *__restrict a0, int *__restrict a1) {
  return pthread_rwlockattr_getpshared(a0, a1);
}

int f406(pthread_rwlockattr_t *a0) { return pthread_rwlockattr_init(a0); }

int f407(pthread_rwlockattr_t *a0, int a1) {
  return pthread_rwlockattr_setkind_np(a0, a1);
}

int f408(pthread_rwlockattr_t *a0, int a1) {
  return pthread_rwlockattr_setpshared(a0, a1);
}

pthread_t f409() { return pthread_self(); }

int f410(pthread_t a0, size_t a1, const cpu_set_t *a2) {
  return pthread_setaffinity_np(a0, a1, a2);
}

int f411(const pthread_attr_t *a0) { return pthread_setattr_default_np(a0); }

int f412(int a0, int *a1) { return pthread_setcancelstate(a0, a1); }

int f413(int a0, int *a1) { return pthread_setcanceltype(a0, a1); }

int f414(int a0) { return pthread_setconcurrency(a0); }

int f415(pthread_t a0, const char *a1) { return pthread_setname_np(a0, a1); }

int f416(pthread_t a0, int a1, const struct sched_param *a2) {
  return pthread_setschedparam(a0, a1, a2);
}

int f417(pthread_t a0, int a1) { return pthread_setschedprio(a0, a1); }

int f418(pthread_key_t a0, const void *a1) {
  return pthread_setspecific(a0, a1);
}

int f419(pthread_spinlock_t *a0) { return pthread_spin_destroy(a0); }

int f420(pthread_spinlock_t *a0, int a1) { return pthread_spin_init(a0, a1); }

int f421(pthread_spinlock_t *a0) { return pthread_spin_lock(a0); }

int f422(pthread_spinlock_t *a0) { return pthread_spin_trylock(a0); }

int f423(pthread_spinlock_t *a0) { return pthread_spin_unlock(a0); }

void f424() { return pthread_testcancel(); }

int f425(pthread_t a0, void **a1, const struct timespec *a2) {
  return pthread_timedjoin_np(a0, a1, a2);
}

int f426(pthread_t a0, void **a1) { return pthread_tryjoin_np(a0, a1); }

ssize_t f427(int a0, const void *a1, size_t a2, long a3) {
  return pwrite(a0, a1, a2, a3);
}

ssize_t f428(int a0, const void *a1, size_t a2, long a3) {
  return pwrite64(a0, a1, a2, a3);
}

ssize_t f429(int a0, void *a1, size_t a2) { return read(a0, a1, a2); }

ssize_t f430(const char *__restrict a0, char *__restrict a1, size_t a2) {
  return readlink(a0, a1, a2);
}

ssize_t f431(int a0, const char *__restrict a1, char *__restrict a2,
             size_t a3) {
  return readlinkat(a0, a1, a2, a3);
}

int f432(const char *a0) { return revoke(a0); }

int f433(const char *a0) { return rmdir(a0); }

void *f434(intptr_t a0) { return sbrk(a0); }

int f435(int a0) { return sched_get_priority_max(a0); }

int f436(int a0) { return sched_get_priority_min(a0); }

int f437(int a0, size_t a1, cpu_set_t *a2) {
  return sched_getaffinity(a0, a1, a2);
}

int f438() { return sched_getcpu(); }

int f439(int a0, struct sched_param *a1) { return sched_getparam(a0, a1); }

int f440(int a0) { return sched_getscheduler(a0); }

using t69 = struct sched_param;

int f441(int a0, struct timespec *a1) { return sched_rr_get_interval(a0, a1); }

int f442(int a0, size_t a1, const cpu_set_t *a2) {
  return sched_setaffinity(a0, a1, a2);
}

int f443(int a0, const struct sched_param *a1) {
  return sched_setparam(a0, a1);
}

int f444(int a0, int a1, const struct sched_param *a2) {
  return sched_setscheduler(a0, a1, a2);
}

int f445() { return sched_yield(); }

int f446(const char *a0, size_t a1) { return setdomainname(a0, a1); }

int f447(unsigned int a0) { return setegid(a0); }

int f448(unsigned int a0) { return seteuid(a0); }

int f449(unsigned int a0) { return setgid(a0); }

int f450(long a0) { return sethostid(a0); }

int f451(const char *a0, size_t a1) { return sethostname(a0, a1); }

int f452(const char *a0) { return setlogin(a0); }

int f453(int a0, int a1) { return setns(a0, a1); }

int f454(int a0, int a1) { return setpgid(a0, a1); }

int f455() { return setpgrp(); }

int f456(unsigned int a0, unsigned int a1) { return setregid(a0, a1); }

int f457(unsigned int a0, unsigned int a1, unsigned int a2) {
  return setresgid(a0, a1, a2);
}

int f458(unsigned int a0, unsigned int a1, unsigned int a2) {
  return setresuid(a0, a1, a2);
}

int f459(unsigned int a0, unsigned int a1) { return setreuid(a0, a1); }

int f460() { return setsid(); }

int f461(unsigned int a0) { return setuid(a0); }

void f462() { return setusershell(); }

unsigned int f463(unsigned int a0) { return sleep(a0); }

using t70 = socklen_t;

void f464(const std::atomic<char> &a0, char a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f465(std::atomic<char> &a0) { return a0.notify_one(); }

void f466(std::atomic<char> &a0) { return a0.notify_all(); }

using t71 = std::atomic<long double>;

std::atomic<long double> f467() { return {}; }

std::atomic<long double> f468(long double a0) { return {a0}; }

bool f469(const std::atomic<long double> &a0) { return a0.is_lock_free(); }

void f470(std::atomic<long double> &a0, long double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

long double f471(const std::atomic<long double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

long double f472(const std::atomic<long double> &a0) {
  return static_cast<long double>(a0);
}

long double f473(std::atomic<long double> &a0, long double a1,
                 std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f474(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f475(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f476(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f477(std::atomic<long double> &a0, long double &a1, long double a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f478(const std::atomic<long double> &a0, long double a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f479(const std::atomic<long double> &a0) { return a0.notify_one(); }

void f480(const std::atomic<long double> &a0) { return a0.notify_all(); }

void f481(const std::atomic<int> &a0, int a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f482(std::atomic<int> &a0) { return a0.notify_one(); }

void f483(std::atomic<int> &a0) { return a0.notify_all(); }

void f484(const std::atomic<long long> &a0, long long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f485(std::atomic<long long> &a0) { return a0.notify_one(); }

void f486(std::atomic<long long> &a0) { return a0.notify_all(); }

void f487(const std::atomic<long> &a0, long a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f488(std::atomic<long> &a0) { return a0.notify_one(); }

void f489(std::atomic<long> &a0) { return a0.notify_all(); }

void f490(const std::atomic<short> &a0, short a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f491(std::atomic<short> &a0) { return a0.notify_one(); }

void f492(std::atomic<short> &a0) { return a0.notify_all(); }

void f493(const std::atomic<wchar_t> &a0, wchar_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f494(std::atomic<wchar_t> &a0) { return a0.notify_one(); }

void f495(std::atomic<wchar_t> &a0) { return a0.notify_all(); }

void f496(const std::atomic<bool> &a0, bool a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f497(std::atomic<bool> &a0) { return a0.notify_one(); }

void f498(std::atomic<bool> &a0) { return a0.notify_all(); }

void f499(const std::atomic<unsigned char> &a0, unsigned char a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f500(std::atomic<unsigned char> &a0) { return a0.notify_one(); }

void f501(std::atomic<unsigned char> &a0) { return a0.notify_all(); }

using t72 = std::atomic<double>;

std::atomic<double> f502() { return {}; }

std::atomic<double> f503(double a0) { return {a0}; }

bool f504(const std::atomic<double> &a0) { return a0.is_lock_free(); }

void f505(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

double f506(const std::atomic<double> &a0, std::memory_order a1) {
  return a0.load(a1);
}

double f507(const std::atomic<double> &a0) { return static_cast<double>(a0); }

double f508(std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f509(std::atomic<double> &a0, double &a1, double a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f510(std::atomic<double> &a0, double &a1, double a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f511(std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f512(std::atomic<double> &a0, double &a1, double a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f513(const std::atomic<double> &a0, double a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f514(const std::atomic<double> &a0) { return a0.notify_one(); }

void f515(const std::atomic<double> &a0) { return a0.notify_all(); }

using t73 = std::atomic<float>;

std::atomic<float> f516() { return {}; }

std::atomic<float> f517(float a0) { return {a0}; }

bool f518(const std::atomic<float> &a0) { return a0.is_lock_free(); }

void f519(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

float f520(const std::atomic<float> &a0, std::memory_order a1) {
  return a0.load(a1);
}

float f521(const std::atomic<float> &a0) { return static_cast<float>(a0); }

float f522(std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

bool f523(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f524(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f525(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f526(std::atomic<float> &a0, float &a1, float a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f527(const std::atomic<float> &a0, float a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f528(const std::atomic<float> &a0) { return a0.notify_one(); }

void f529(const std::atomic<float> &a0) { return a0.notify_all(); }

void f530(const std::atomic<unsigned int> &a0, unsigned int a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f531(std::atomic<unsigned int> &a0) { return a0.notify_one(); }

void f532(std::atomic<unsigned int> &a0) { return a0.notify_all(); }

void f533(const std::atomic<unsigned long long> &a0, unsigned long long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f534(std::atomic<unsigned long long> &a0) { return a0.notify_one(); }

void f535(std::atomic<unsigned long long> &a0) { return a0.notify_all(); }

void f536(const std::atomic<unsigned long> &a0, unsigned long a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f537(std::atomic<unsigned long> &a0) { return a0.notify_one(); }

void f538(std::atomic<unsigned long> &a0) { return a0.notify_all(); }

void f539(const std::atomic<char16_t> &a0, char16_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f540(std::atomic<char16_t> &a0) { return a0.notify_one(); }

void f541(std::atomic<char16_t> &a0) { return a0.notify_all(); }

void f542(const std::atomic<signed char> &a0, signed char a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f543(std::atomic<signed char> &a0) { return a0.notify_one(); }

void f544(std::atomic<signed char> &a0) { return a0.notify_all(); }

void f545(const std::atomic<unsigned short> &a0, unsigned short a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f546(std::atomic<unsigned short> &a0) { return a0.notify_one(); }

void f547(std::atomic<unsigned short> &a0) { return a0.notify_all(); }

using t74 = std::atomic<char8_t>;

std::atomic<char8_t> f548() { return {}; }

std::atomic<char8_t> f549(char8_t a0) { return {a0}; }

bool f550(const std::atomic<char8_t> &a0) { return a0.is_lock_free(); }

void f551(std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

bool f552(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

bool f553(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

bool f554(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

bool f555(std::atomic<char8_t> &a0, char8_t &a1, char8_t a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

void f556(const std::atomic<char8_t> &a0, char8_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f557(std::atomic<char8_t> &a0) { return a0.notify_one(); }

void f558(std::atomic<char8_t> &a0) { return a0.notify_all(); }

bool f559() { return std::atomic<char8_t>::is_always_lock_free; }

void f560(const std::atomic<char32_t> &a0, char32_t a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f561(std::atomic<char32_t> &a0) { return a0.notify_one(); }

void f562(std::atomic<char32_t> &a0) { return a0.notify_all(); }

template <typename T1>
void f563(const std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f564(std::atomic<T1 *> &a0) {
  return a0.notify_one();
}

template <typename T1> void f565(std::atomic<T1 *> &a0) {
  return a0.notify_all();
}

template <typename T1> using t75 = std::atomic<std::shared_ptr<T1>>;

template <typename T1>
using t76 = typename std::atomic<std::shared_ptr<T1>>::value_type;

template <typename T1> bool f566(const std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.is_lock_free();
}

template <typename T1> std::atomic<std::shared_ptr<T1>> f567() { return {}; }

template <typename T1>
std::atomic<std::shared_ptr<T1>> f568(std::nullptr_t a0) {
  return {a0};
}

template <typename T1>
std::atomic<std::shared_ptr<T1>> f569(std::shared_ptr<T1> a0) {
  return {a0};
}

template <typename T1>
std::shared_ptr<T1> f570(const std::atomic<std::shared_ptr<T1>> &a0,
                         std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
std::shared_ptr<T1> f571(const std::atomic<std::shared_ptr<T1>> &a0) {
  return static_cast<std::shared_ptr<T1>>(a0);
}

template <typename T1>
void f572(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
void f573(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1>
void f574(std::atomic<std::shared_ptr<T1>> &a0, std::nullptr_t a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::shared_ptr<T1> f575(std::atomic<std::shared_ptr<T1>> &a0,
                         std::shared_ptr<T1> a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f576(std::atomic<std::shared_ptr<T1>> &a0, std::shared_ptr<T1> &a1,
          std::shared_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f577(std::atomic<std::shared_ptr<T1>> &a0,
          typename std::atomic<std::shared_ptr<T1>>::value_type &a1,
          typename std::atomic<std::shared_ptr<T1>>::value_type a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
bool f578(std::atomic<std::shared_ptr<T1>> &a0,
          typename std::atomic<std::shared_ptr<T1>>::value_type &a1,
          typename std::atomic<std::shared_ptr<T1>>::value_type a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f579(std::atomic<std::shared_ptr<T1>> &a0,
          typename std::atomic<std::shared_ptr<T1>>::value_type &a1,
          typename std::atomic<std::shared_ptr<T1>>::value_type a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
void f580(const std::atomic<std::shared_ptr<T1>> &a0,
          typename std::atomic<std::shared_ptr<T1>>::value_type a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f581(std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.notify_one();
}

template <typename T1> void f582(std::atomic<std::shared_ptr<T1>> &a0) {
  return a0.notify_all();
}

template <typename T1>
using t77 = typename std::atomic<std::shared_ptr<T1>>::value_type::element_type;

template <typename T1>
using t78 = typename std::atomic<std::shared_ptr<T1>>::value_type::weak_type;

template <typename T1> bool f583() {
  return std::atomic<std::shared_ptr<T1>>::is_always_lock_free;
}

template <typename T1> using t79 = std::atomic<std::weak_ptr<T1>>;

template <typename T1>
using t80 = typename std::atomic<std::weak_ptr<T1>>::value_type;

template <typename T1> bool f584(const std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.is_lock_free();
}

template <typename T1> std::atomic<std::weak_ptr<T1>> f585() { return {}; }

template <typename T1>
std::atomic<std::weak_ptr<T1>> f586(std::weak_ptr<T1> a0) {
  return {a0};
}

template <typename T1>
std::weak_ptr<T1> f587(const std::atomic<std::weak_ptr<T1>> &a0,
                       std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
std::weak_ptr<T1> f588(const std::atomic<std::weak_ptr<T1>> &a0) {
  return static_cast<std::weak_ptr<T1>>(a0);
}

template <typename T1>
void f589(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
          std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
void f590(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::weak_ptr<T1> f591(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> a1,
                       std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f592(std::atomic<std::weak_ptr<T1>> &a0, std::weak_ptr<T1> &a1,
          std::weak_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f593(std::atomic<std::weak_ptr<T1>> &a0,
          typename std::atomic<std::weak_ptr<T1>>::value_type &a1,
          typename std::atomic<std::weak_ptr<T1>>::value_type a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
bool f594(std::atomic<std::weak_ptr<T1>> &a0,
          typename std::atomic<std::weak_ptr<T1>>::value_type &a1,
          typename std::atomic<std::weak_ptr<T1>>::value_type a2,
          std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f595(std::atomic<std::weak_ptr<T1>> &a0,
          typename std::atomic<std::weak_ptr<T1>>::value_type &a1,
          typename std::atomic<std::weak_ptr<T1>>::value_type a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
void f596(const std::atomic<std::weak_ptr<T1>> &a0,
          typename std::atomic<std::weak_ptr<T1>>::value_type a1,
          std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f597(std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.notify_one();
}

template <typename T1> void f598(std::atomic<std::weak_ptr<T1>> &a0) {
  return a0.notify_all();
}

template <typename T1>
std::shared_ptr<T1>
f599(const typename std::atomic<std::weak_ptr<T1>>::value_type &a0) {
  return a0.lock();
}

template <typename T1> bool f600() {
  return std::atomic<std::weak_ptr<T1>>::is_always_lock_free;
}

template <typename T1>
void f601(const std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f602(std::atomic<T1> &a0) {
  return a0.notify_one();
}

template <typename T1> void f603(std::atomic<T1> &a0) {
  return a0.notify_all();
}

using t81 = std::atomic_char8_t;

void f604(std::atomic_flag *a0) { return std::atomic_flag_notify_all(a0); }

void f605(std::atomic_flag *a0) { return std::atomic_flag_notify_one(a0); }

bool f606(const std::atomic_flag *a0) { return std::atomic_flag_test(a0); }

bool f607(const volatile std::atomic_flag *a0) {
  return std::atomic_flag_test(a0);
}

bool f608(const std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}

bool f609(const volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}

void f610(std::atomic_flag *a0, bool a1) {
  return std::atomic_flag_wait(a0, a1);
}

void f611(std::atomic_flag *a0, bool a1, std::memory_order a2) {
  return std::atomic_flag_wait_explicit(a0, a1, a2);
}

template <typename T1> void f612(std::atomic<T1> *a0) {
  return std::atomic_notify_all<T1>(a0);
}

template <typename T1> void f613(std::atomic<T1> *a0) {
  return std::atomic_notify_one<T1>(a0);
}

using t82 = std::atomic_signed_lock_free;

using t83 = std::atomic_unsigned_lock_free;

template <typename T1>
void f614(const std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_wait<T1>(a0, a1);
}

template <typename T1>
void f615(const std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
          std::memory_order a2) {
  return std::atomic_wait_explicit<T1>(a0, a1, a2);
}

size_t f616(char *__restrict a0, size_t a1, const char *__restrict a2,
            const struct tm *__restrict a3, locale_t a4) {
  return strftime_l(a0, a1, a2, a3, a4);
}

char *f617(const char *__restrict a0, const char *__restrict a1,
           struct tm *a2) {
  return strptime(a0, a1, a2);
}

char *f618(const char *__restrict a0, const char *__restrict a1, struct tm *a2,
           locale_t a3) {
  return strptime_l(a0, a1, a2, a3);
}

void f619(const void *__restrict a0, void *__restrict a1, ssize_t a2) {
  return swab(a0, a1, a2);
}

int f620(const char *a0, const char *a1) { return symlink(a0, a1); }

int f621(const char *a0, int a1, const char *a2) {
  return symlinkat(a0, a1, a2);
}

void f622() { return sync(); }

int f623(int a0) { return syncfs(a0); }

template <typename... Args> long f624(long a0, Args... args) {
  return syscall(a0, args...);
}

long f625(int a0) { return sysconf(a0); }

int f626(int a0) { return tcgetpgrp(a0); }

int f627(int a0, int a1) { return tcsetpgrp(a0, a1); }

time_t f628(struct tm *a0) { return timegm(a0); }

time_t f629(struct tm *a0) { return timelocal(a0); }

int f630(clockid_t a0, struct sigevent *__restrict a1, timer_t *__restrict a2) {
  return timer_create(a0, a1, a2);
}

int f631(timer_t a0) { return timer_delete(a0); }

int f632(timer_t a0) { return timer_getoverrun(a0); }

int f633(timer_t a0, struct itimerspec *a1) { return timer_gettime(a0, a1); }

int f634(timer_t a0, int a1, const struct itimerspec *__restrict a2,
         struct itimerspec *__restrict a3) {
  return timer_settime(a0, a1, a2, a3);
}

int f635(struct timespec *a0, int a1) { return timespec_getres(a0, a1); }

using t84 = struct timex;

using t85 = struct tm;

int f636(const char *a0, long a1) { return truncate(a0, a1); }

int f637(const char *a0, long a1) { return truncate64(a0, a1); }

char *f638(int a0) { return ttyname(a0); }

int f639(int a0, char *a1, size_t a2) { return ttyname_r(a0, a1, a2); }

int f640() { return ttyslot(); }

void f641() { return tzset(); }

unsigned int f642(unsigned int a0, unsigned int a1) { return ualarm(a0, a1); }

int f643(const char *a0) { return unlink(a0); }

int f644(int a0, const char *a1, int a2) { return unlinkat(a0, a1, a2); }

int f645(int a0) { return unshare(a0); }

int f646(unsigned int a0) { return usleep(a0); }

int f647() { return vfork(); }

int f648() { return vhangup(); }

ssize_t f649(int a0, const void *a1, size_t a2) { return write(a0, a1, a2); }
