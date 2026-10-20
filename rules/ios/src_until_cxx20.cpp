#include "../common/init.h"
#include "../common/traits.h"
#include <ios>
#include <stdlib.h>
#include <utility>

char *f1(const struct tm *a0) { return asctime(a0); }

char *f2(const struct tm *__restrict a0, char *__restrict a1) {
  return asctime_r(a0, a1);
}

clock_t f3() { return clock(); }

int f4(int a0, struct timex *a1) { return clock_adjtime(a0, a1); }

int f5(pid_t a0, clockid_t *a1) { return clock_getcpuclockid(a0, a1); }

int f6(clockid_t a0, struct timespec *a1) { return clock_getres(a0, a1); }

int f7(clockid_t a0, struct timespec *a1) { return clock_gettime(a0, a1); }

int f8(clockid_t a0, int a1, const struct timespec *a2, struct timespec *a3) {
  return clock_nanosleep(a0, a1, a2, a3);
}

int f9(clockid_t a0, const struct timespec *a1) {
  return clock_settime(a0, a1);
}

template <typename... Args>
int f10(int (*a0)(void *), void *a1, int a2, void *a3, Args... args) {
  return clone(a0, a1, a2, a3, args...);
}

using t1 = cpu_set_t;

char *f11(const time_t *a0) { return ctime(a0); }

char *f12(const time_t *__restrict a0, char *__restrict a1) {
  return ctime_r(a0, a1);
}

double f13(time_t a0, time_t a1) { return difftime(a0, a1); }

int f14(int a0) { return dysize(a0); }

int f15(unsigned int *a0, unsigned int *a1) { return getcpu(a0, a1); }

struct tm *f16(const char *a0) { return getdate(a0); }

int f17(const char *__restrict a0, struct tm *__restrict a1) {
  return getdate_r(a0, a1);
}

struct tm *f18(const time_t *a0) { return gmtime(a0); }

struct tm *f19(const time_t *__restrict a0, struct tm *__restrict a1) {
  return gmtime_r(a0, a1);
}

using t2 = struct itimerspec;

struct tm *f20(const time_t *a0) { return localtime(a0); }

struct tm *f21(const time_t *__restrict a0, struct tm *__restrict a1) {
  return localtime_r(a0, a1);
}

time_t f22(struct tm *a0) { return mktime(a0); }

int f23(const struct timespec *a0, struct timespec *a1) {
  return nanosleep(a0, a1);
}

int f24(void (*a0)(), void (*a1)(), void (*a2)()) {
  return pthread_atfork(a0, a1, a2);
}

int f25(pthread_attr_t *a0) { return pthread_attr_destroy(a0); }

int f26(const pthread_attr_t *a0, size_t a1, cpu_set_t *a2) {
  return pthread_attr_getaffinity_np(a0, a1, a2);
}

int f27(const pthread_attr_t *a0, int *a1) {
  return pthread_attr_getdetachstate(a0, a1);
}

int f28(const pthread_attr_t *a0, size_t *a1) {
  return pthread_attr_getguardsize(a0, a1);
}

int f29(const pthread_attr_t *__restrict a0, int *__restrict a1) {
  return pthread_attr_getinheritsched(a0, a1);
}

int f30(const pthread_attr_t *__restrict a0,
        struct sched_param *__restrict a1) {
  return pthread_attr_getschedparam(a0, a1);
}

int f31(const pthread_attr_t *__restrict a0, int *__restrict a1) {
  return pthread_attr_getschedpolicy(a0, a1);
}

int f32(const pthread_attr_t *__restrict a0, int *__restrict a1) {
  return pthread_attr_getscope(a0, a1);
}

int f33(const pthread_attr_t *a0, sigset_t *a1) {
  return pthread_attr_getsigmask_np(a0, a1);
}

int f34(const pthread_attr_t *__restrict a0, void **__restrict a1,
        size_t *__restrict a2) {
  return pthread_attr_getstack(a0, a1, a2);
}

int f35(const pthread_attr_t *__restrict a0, size_t *__restrict a1) {
  return pthread_attr_getstacksize(a0, a1);
}

int f36(pthread_attr_t *a0) { return pthread_attr_init(a0); }

int f37(pthread_attr_t *a0, size_t a1, const cpu_set_t *a2) {
  return pthread_attr_setaffinity_np(a0, a1, a2);
}

int f38(pthread_attr_t *a0, int a1) {
  return pthread_attr_setdetachstate(a0, a1);
}

int f39(pthread_attr_t *a0, size_t a1) {
  return pthread_attr_setguardsize(a0, a1);
}

int f40(pthread_attr_t *a0, int a1) {
  return pthread_attr_setinheritsched(a0, a1);
}

int f41(pthread_attr_t *__restrict a0,
        const struct sched_param *__restrict a1) {
  return pthread_attr_setschedparam(a0, a1);
}

int f42(pthread_attr_t *a0, int a1) {
  return pthread_attr_setschedpolicy(a0, a1);
}

int f43(pthread_attr_t *a0, int a1) { return pthread_attr_setscope(a0, a1); }

int f44(pthread_attr_t *a0, const sigset_t *a1) {
  return pthread_attr_setsigmask_np(a0, a1);
}

int f45(pthread_attr_t *a0, void *a1, size_t a2) {
  return pthread_attr_setstack(a0, a1, a2);
}

int f46(pthread_attr_t *a0, size_t a1) {
  return pthread_attr_setstacksize(a0, a1);
}

int f47(pthread_barrier_t *a0) { return pthread_barrier_destroy(a0); }

int f48(pthread_barrier_t *__restrict a0,
        const pthread_barrierattr_t *__restrict a1, unsigned int a2) {
  return pthread_barrier_init(a0, a1, a2);
}

int f49(pthread_barrier_t *a0) { return pthread_barrier_wait(a0); }

int f50(pthread_barrierattr_t *a0) { return pthread_barrierattr_destroy(a0); }

int f51(const pthread_barrierattr_t *__restrict a0, int *__restrict a1) {
  return pthread_barrierattr_getpshared(a0, a1);
}

int f52(pthread_barrierattr_t *a0) { return pthread_barrierattr_init(a0); }

int f53(pthread_barrierattr_t *a0, int a1) {
  return pthread_barrierattr_setpshared(a0, a1);
}

int f54(pthread_t a0) { return pthread_cancel(a0); }

int f55(pthread_t a0, void **a1, clockid_t a2, const struct timespec *a3) {
  return pthread_clockjoin_np(a0, a1, a2, a3);
}

int f56(pthread_cond_t *a0) { return pthread_cond_broadcast(a0); }

int f57(pthread_cond_t *__restrict a0, pthread_mutex_t *__restrict a1, int a2,
        const struct timespec *__restrict a3) {
  return pthread_cond_clockwait(a0, a1, a2, a3);
}

int f58(pthread_cond_t *a0) { return pthread_cond_destroy(a0); }

int f59(pthread_cond_t *__restrict a0,
        const pthread_condattr_t *__restrict a1) {
  return pthread_cond_init(a0, a1);
}

int f60(pthread_cond_t *a0) { return pthread_cond_signal(a0); }

int f61(pthread_cond_t *__restrict a0, pthread_mutex_t *__restrict a1,
        const struct timespec *__restrict a2) {
  return pthread_cond_timedwait(a0, a1, a2);
}

int f62(pthread_cond_t *__restrict a0, pthread_mutex_t *__restrict a1) {
  return pthread_cond_wait(a0, a1);
}

int f63(pthread_condattr_t *a0) { return pthread_condattr_destroy(a0); }

int f64(const pthread_condattr_t *__restrict a0, int *__restrict a1) {
  return pthread_condattr_getclock(a0, a1);
}

int f65(const pthread_condattr_t *__restrict a0, int *__restrict a1) {
  return pthread_condattr_getpshared(a0, a1);
}

int f66(pthread_condattr_t *a0) { return pthread_condattr_init(a0); }

int f67(pthread_condattr_t *a0, int a1) {
  return pthread_condattr_setclock(a0, a1);
}

int f68(pthread_condattr_t *a0, int a1) {
  return pthread_condattr_setpshared(a0, a1);
}

int f69(pthread_t *__restrict a0, const pthread_attr_t *__restrict a1,
        void *(*a2)(void *), void *__restrict a3) {
  return pthread_create(a0, a1, a2, a3);
}

int f70(pthread_t a0) { return pthread_detach(a0); }

int f71(pthread_t a0, pthread_t a1) { return pthread_equal(a0, a1); }

void f72(void *a0) { return pthread_exit(a0); }

int f73(pthread_t a0, size_t a1, cpu_set_t *a2) {
  return pthread_getaffinity_np(a0, a1, a2);
}

int f74(pthread_attr_t *a0) { return pthread_getattr_default_np(a0); }

int f75(pthread_t a0, pthread_attr_t *a1) { return pthread_getattr_np(a0, a1); }

int f76() { return pthread_getconcurrency(); }

int f77(pthread_t a0, int *a1) { return pthread_getcpuclockid(a0, a1); }

int f78(pthread_t a0, char *a1, size_t a2) {
  return pthread_getname_np(a0, a1, a2);
}

int f79(pthread_t a0, int *__restrict a1, struct sched_param *__restrict a2) {
  return pthread_getschedparam(a0, a1, a2);
}

void *f80(pthread_key_t a0) { return pthread_getspecific(a0); }

int f81(pthread_t a0, void **a1) { return pthread_join(a0, a1); }

int f82(pthread_key_t *a0, void (*a1)(void *)) {
  return pthread_key_create(a0, a1);
}

int f83(pthread_key_t a0) { return pthread_key_delete(a0); }

int f84(pthread_mutex_t *__restrict a0, clockid_t a1,
        const struct timespec *__restrict a2) {
  return pthread_mutex_clocklock(a0, a1, a2);
}

int f85(pthread_mutex_t *a0) { return pthread_mutex_consistent(a0); }

int f86(pthread_mutex_t *a0) { return pthread_mutex_destroy(a0); }

int f87(const pthread_mutex_t *__restrict a0, int *__restrict a1) {
  return pthread_mutex_getprioceiling(a0, a1);
}

int f88(pthread_mutex_t *a0, const pthread_mutexattr_t *a1) {
  return pthread_mutex_init(a0, a1);
}

int f89(pthread_mutex_t *a0) { return pthread_mutex_lock(a0); }

int f90(pthread_mutex_t *__restrict a0, int a1, int *__restrict a2) {
  return pthread_mutex_setprioceiling(a0, a1, a2);
}

int f91(pthread_mutex_t *__restrict a0, const struct timespec *__restrict a1) {
  return pthread_mutex_timedlock(a0, a1);
}

int f92(pthread_mutex_t *a0) { return pthread_mutex_trylock(a0); }

int f93(pthread_mutex_t *a0) { return pthread_mutex_unlock(a0); }

int f94(pthread_mutexattr_t *a0) { return pthread_mutexattr_destroy(a0); }

int f95(const pthread_mutexattr_t *__restrict a0, int *__restrict a1) {
  return pthread_mutexattr_getprioceiling(a0, a1);
}

int f96(const pthread_mutexattr_t *__restrict a0, int *__restrict a1) {
  return pthread_mutexattr_getprotocol(a0, a1);
}

int f97(const pthread_mutexattr_t *__restrict a0, int *__restrict a1) {
  return pthread_mutexattr_getpshared(a0, a1);
}

int f98(const pthread_mutexattr_t *a0, int *a1) {
  return pthread_mutexattr_getrobust(a0, a1);
}

int f99(const pthread_mutexattr_t *__restrict a0, int *__restrict a1) {
  return pthread_mutexattr_gettype(a0, a1);
}

int f100(pthread_mutexattr_t *a0) { return pthread_mutexattr_init(a0); }

int f101(pthread_mutexattr_t *a0, int a1) {
  return pthread_mutexattr_setprioceiling(a0, a1);
}

int f102(pthread_mutexattr_t *a0, int a1) {
  return pthread_mutexattr_setprotocol(a0, a1);
}

int f103(pthread_mutexattr_t *a0, int a1) {
  return pthread_mutexattr_setpshared(a0, a1);
}

int f104(pthread_mutexattr_t *a0, int a1) {
  return pthread_mutexattr_setrobust(a0, a1);
}

int f105(pthread_mutexattr_t *a0, int a1) {
  return pthread_mutexattr_settype(a0, a1);
}

int f106(pthread_once_t *a0, void (*a1)()) { return pthread_once(a0, a1); }

int f107(pthread_rwlock_t *__restrict a0, clockid_t a1,
         const struct timespec *__restrict a2) {
  return pthread_rwlock_clockrdlock(a0, a1, a2);
}

int f108(pthread_rwlock_t *__restrict a0, clockid_t a1,
         const struct timespec *__restrict a2) {
  return pthread_rwlock_clockwrlock(a0, a1, a2);
}

int f109(pthread_rwlock_t *a0) { return pthread_rwlock_destroy(a0); }

int f110(pthread_rwlock_t *__restrict a0,
         const pthread_rwlockattr_t *__restrict a1) {
  return pthread_rwlock_init(a0, a1);
}

int f111(pthread_rwlock_t *a0) { return pthread_rwlock_rdlock(a0); }

int f112(pthread_rwlock_t *__restrict a0,
         const struct timespec *__restrict a1) {
  return pthread_rwlock_timedrdlock(a0, a1);
}

int f113(pthread_rwlock_t *__restrict a0,
         const struct timespec *__restrict a1) {
  return pthread_rwlock_timedwrlock(a0, a1);
}

int f114(pthread_rwlock_t *a0) { return pthread_rwlock_tryrdlock(a0); }

int f115(pthread_rwlock_t *a0) { return pthread_rwlock_trywrlock(a0); }

int f116(pthread_rwlock_t *a0) { return pthread_rwlock_unlock(a0); }

int f117(pthread_rwlock_t *a0) { return pthread_rwlock_wrlock(a0); }

int f118(pthread_rwlockattr_t *a0) { return pthread_rwlockattr_destroy(a0); }

int f119(const pthread_rwlockattr_t *__restrict a0, int *__restrict a1) {
  return pthread_rwlockattr_getkind_np(a0, a1);
}

int f120(const pthread_rwlockattr_t *__restrict a0, int *__restrict a1) {
  return pthread_rwlockattr_getpshared(a0, a1);
}

int f121(pthread_rwlockattr_t *a0) { return pthread_rwlockattr_init(a0); }

int f122(pthread_rwlockattr_t *a0, int a1) {
  return pthread_rwlockattr_setkind_np(a0, a1);
}

int f123(pthread_rwlockattr_t *a0, int a1) {
  return pthread_rwlockattr_setpshared(a0, a1);
}

pthread_t f124() { return pthread_self(); }

int f125(pthread_t a0, size_t a1, const cpu_set_t *a2) {
  return pthread_setaffinity_np(a0, a1, a2);
}

int f126(const pthread_attr_t *a0) { return pthread_setattr_default_np(a0); }

int f127(int a0, int *a1) { return pthread_setcancelstate(a0, a1); }

int f128(int a0, int *a1) { return pthread_setcanceltype(a0, a1); }

int f129(int a0) { return pthread_setconcurrency(a0); }

int f130(pthread_t a0, const char *a1) { return pthread_setname_np(a0, a1); }

int f131(pthread_t a0, int a1, const struct sched_param *a2) {
  return pthread_setschedparam(a0, a1, a2);
}

int f132(pthread_t a0, int a1) { return pthread_setschedprio(a0, a1); }

int f133(pthread_key_t a0, const void *a1) {
  return pthread_setspecific(a0, a1);
}

int f134(pthread_spinlock_t *a0) { return pthread_spin_destroy(a0); }

int f135(pthread_spinlock_t *a0, int a1) { return pthread_spin_init(a0, a1); }

int f136(pthread_spinlock_t *a0) { return pthread_spin_lock(a0); }

int f137(pthread_spinlock_t *a0) { return pthread_spin_trylock(a0); }

int f138(pthread_spinlock_t *a0) { return pthread_spin_unlock(a0); }

void f139() { return pthread_testcancel(); }

int f140(pthread_t a0, void **a1, const struct timespec *a2) {
  return pthread_timedjoin_np(a0, a1, a2);
}

int f141(pthread_t a0, void **a1) { return pthread_tryjoin_np(a0, a1); }

int f142(int a0) { return sched_get_priority_max(a0); }

int f143(int a0) { return sched_get_priority_min(a0); }

int f144(int a0, size_t a1, cpu_set_t *a2) {
  return sched_getaffinity(a0, a1, a2);
}

int f145() { return sched_getcpu(); }

int f146(int a0, struct sched_param *a1) { return sched_getparam(a0, a1); }

int f147(int a0) { return sched_getscheduler(a0); }

using t3 = struct sched_param;

int f148(int a0, struct timespec *a1) { return sched_rr_get_interval(a0, a1); }

int f149(int a0, size_t a1, const cpu_set_t *a2) {
  return sched_setaffinity(a0, a1, a2);
}

int f150(int a0, const struct sched_param *a1) {
  return sched_setparam(a0, a1);
}

int f151(int a0, int a1, const struct sched_param *a2) {
  return sched_setscheduler(a0, a1, a2);
}

int f152() { return sched_yield(); }

int f153(int a0, int a1) { return setns(a0, a1); }

size_t f154(char *__restrict a0, size_t a1, const char *__restrict a2,
            const struct tm *__restrict a3) {
  return strftime(a0, a1, a2, a3);
}

size_t f155(char *__restrict a0, size_t a1, const char *__restrict a2,
            const struct tm *__restrict a3, locale_t a4) {
  return strftime_l(a0, a1, a2, a3, a4);
}

char *f156(const char *__restrict a0, const char *__restrict a1,
           struct tm *a2) {
  return strptime(a0, a1, a2);
}

char *f157(const char *__restrict a0, const char *__restrict a1, struct tm *a2,
           locale_t a3) {
  return strptime_l(a0, a1, a2, a3);
}

time_t f158(time_t *a0) { return time(a0); }

time_t f159(struct tm *a0) { return timegm(a0); }

time_t f160(struct tm *a0) { return timelocal(a0); }

int f161(clockid_t a0, struct sigevent *__restrict a1, timer_t *__restrict a2) {
  return timer_create(a0, a1, a2);
}

int f162(timer_t a0) { return timer_delete(a0); }

int f163(timer_t a0) { return timer_getoverrun(a0); }

int f164(timer_t a0, struct itimerspec *a1) { return timer_gettime(a0, a1); }

int f165(timer_t a0, int a1, const struct itimerspec *__restrict a2,
         struct itimerspec *__restrict a3) {
  return timer_settime(a0, a1, a2, a3);
}

int f166(struct timespec *a0, int a1) { return timespec_get(a0, a1); }

int f167(struct timespec *a0, int a1) { return timespec_getres(a0, a1); }

using t4 = struct timex;

using t5 = struct tm;

void f168() { return tzset(); }

int f169(int a0) { return unshare(a0); }
