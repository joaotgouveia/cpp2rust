#include "../common/init.h"
#include "../common/traits.h"
#include <cstdlib>
#include <stdlib.h>
#include <utility>

long f1(const char *a0) { return a64l(a0); }

void f2() { return abort(); }

void *f3(size_t a0, size_t a1) { return aligned_alloc(a0, a1); }

void *f4(size_t a0) { return alloca(a0); }

unsigned int f5() { return arc4random(); }

void f6(void *a0, size_t a1) { return arc4random_buf(a0, a1); }

unsigned int f7(unsigned int a0) { return arc4random_uniform(a0); }

int f8(void (*a0)()) { return at_quick_exit(a0); }

int f9(void (*a0)()) { return atexit(a0); }

double f10(const char *a0) { return atof(a0); }

int f11(const char *a0) { return atoi(a0); }

long f12(const char *a0) { return atol(a0); }

long long f13(const char *a0) { return atoll(a0); }

using t1 = blkcnt64_t;

using t2 = blkcnt_t;

using t3 = blksize_t;

void *f14(const void *a0, const void *a1, size_t a2, size_t a3,
          int (*a4)(const void *, const void *)) {
  return bsearch(a0, a1, a2, a3, a4);
}

using t4 = caddr_t;

void *f15(size_t a0, size_t a1) { return calloc(a0, a1); }

char *f16(const char *a0) { return canonicalize_file_name(a0); }

int f17() { return clearenv(); }

using t5 = clockid_t;

using t6 = comparison_fn_t;

using t7 = daddr_t;

using t8 = dev_t;

div_t f18(int a0, int a1) { return div(a0, a1); }

using t9 = div_t;

double f19() { return drand48(); }

using t10 = struct drand48_data;

int f20(struct drand48_data *__restrict a0, double *__restrict a1) {
  return drand48_r(a0, a1);
}

char *f21(double a0, int a1, int *__restrict a2, int *__restrict a3) {
  return ecvt(a0, a1, a2, a3);
}

int f22(double a0, int a1, int *__restrict a2, int *__restrict a3,
        char *__restrict a4, size_t a5) {
  return ecvt_r(a0, a1, a2, a3, a4, a5);
}

double f23(unsigned short a0[3]) { return erand48(a0); }

int f24(unsigned short a0[3], struct drand48_data *__restrict a1,
        double *__restrict a2) {
  return erand48_r(a0, a1, a2);
}

void f25(int a0) { return exit(a0); }

char *f26(double a0, int a1, int *__restrict a2, int *__restrict a3) {
  return fcvt(a0, a1, a2, a3);
}

int f27(double a0, int a1, int *__restrict a2, int *__restrict a3,
        char *__restrict a4, size_t a5) {
  return fcvt_r(a0, a1, a2, a3, a4, a5);
}

using t11 = fd_mask;

using t12 = fd_set;

void f28(void *a0) { return free(a0); }

using t13 = fsblkcnt64_t;

using t14 = fsblkcnt_t;

using t15 = fsfilcnt64_t;

using t16 = fsfilcnt_t;

using t17 = fsid_t;

char *f29(double a0, int a1, char *a2) { return gcvt(a0, a1, a2); }

char *f30(const char *a0) { return getenv(a0); }

int f31(double a0[], int a1) { return getloadavg(a0, a1); }

int f32() { return getpt(); }

int f33(char **__restrict a0, char *const *__restrict a1,
        char **__restrict a2) {
  return getsubopt(a0, a1, a2);
}

using t18 = gid_t;

int f34(int a0) { return grantpt(a0); }

using t19 = id_t;

char *f35(unsigned int a0, char *a1, size_t a2) {
  return initstate(a0, a1, a2);
}

int f36(unsigned int a0, char *__restrict a1, size_t a2,
        struct random_data *__restrict a3) {
  return initstate_r(a0, a1, a2, a3);
}

using t20 = ino64_t;

using t21 = ino_t;

long f37(unsigned short a0[3]) { return jrand48(a0); }

int f38(unsigned short a0[3], struct drand48_data *__restrict a1,
        long *__restrict a2) {
  return jrand48_r(a0, a1, a2);
}

using t22 = key_t;

char *f39(long a0) { return l64a(a0); }

long f40(long a0) { return labs(a0); }

void f41(unsigned short a0[7]) { return lcong48(a0); }

int f42(unsigned short a0[7], struct drand48_data *a1) {
  return lcong48_r(a0, a1);
}

ldiv_t f43(long a0, long a1) { return ldiv(a0, a1); }

using t23 = ldiv_t;

long long f44(long long a0) { return llabs(a0); }

lldiv_t f45(long long a0, long long a1) { return lldiv(a0, a1); }

using t24 = lldiv_t;

using t25 = locale_t;

using t26 = loff_t;

long f46() { return lrand48(); }

int f47(struct drand48_data *__restrict a0, long *__restrict a1) {
  return lrand48_r(a0, a1);
}

void *f48(size_t a0) { return malloc(a0); }

int f49(const char *a0, size_t a1) { return mblen(a0, a1); }

size_t f50(wchar_t *__restrict a0, const char *__restrict a1, size_t a2) {
  return mbstowcs(a0, a1, a2);
}

int f51(wchar_t *__restrict a0, const char *__restrict a1, size_t a2) {
  return mbtowc(a0, a1, a2);
}

char *f52(char *a0) { return mkdtemp(a0); }

int f53(char *a0, int a1) { return mkostemp(a0, a1); }

int f54(char *a0, int a1) { return mkostemp64(a0, a1); }

int f55(char *a0, int a1, int a2) { return mkostemps(a0, a1, a2); }

int f56(char *a0, int a1, int a2) { return mkostemps64(a0, a1, a2); }

int f57(char *a0) { return mkstemp(a0); }

int f58(char *a0) { return mkstemp64(a0); }

int f59(char *a0, int a1) { return mkstemps(a0, a1); }

int f60(char *a0, int a1) { return mkstemps64(a0, a1); }

char *f61(char *a0) { return mktemp(a0); }

using t27 = mode_t;

long f62() { return mrand48(); }

int f63(struct drand48_data *__restrict a0, long *__restrict a1) {
  return mrand48_r(a0, a1);
}

using t28 = nlink_t;

long f64(unsigned short a0[3]) { return nrand48(a0); }

int f65(unsigned short a0[3], struct drand48_data *__restrict a1,
        long *__restrict a2) {
  return nrand48_r(a0, a1, a2);
}

using t29 = off64_t;

using t30 = off_t;

int f66(void (*a0)(int, void *), void *a1) { return on_exit(a0, a1); }

using t31 = pid_t;

int f67(void **a0, size_t a1, size_t a2) { return posix_memalign(a0, a1, a2); }

int f68(int a0) { return posix_openpt(a0); }

using t32 = pthread_attr_t;

using t33 = union pthread_attr_t;

using t34 = pthread_barrier_t;

using t35 = pthread_barrierattr_t;

using t36 = pthread_cond_t;

using t37 = pthread_condattr_t;

using t38 = pthread_key_t;

using t39 = pthread_mutex_t;

using t40 = pthread_mutexattr_t;

using t41 = pthread_once_t;

using t42 = pthread_rwlock_t;

using t43 = pthread_rwlockattr_t;

using t44 = pthread_spinlock_t;

using t45 = pthread_t;

char *f69(int a0) { return ptsname(a0); }

int f70(int a0, char *a1, size_t a2) { return ptsname_r(a0, a1, a2); }

int f71(char *a0) { return putenv(a0); }

char *f72(long double a0, int a1, int *__restrict a2, int *__restrict a3) {
  return qecvt(a0, a1, a2, a3);
}

int f73(long double a0, int a1, int *__restrict a2, int *__restrict a3,
        char *__restrict a4, size_t a5) {
  return qecvt_r(a0, a1, a2, a3, a4, a5);
}

char *f74(long double a0, int a1, int *__restrict a2, int *__restrict a3) {
  return qfcvt(a0, a1, a2, a3);
}

int f75(long double a0, int a1, int *__restrict a2, int *__restrict a3,
        char *__restrict a4, size_t a5) {
  return qfcvt_r(a0, a1, a2, a3, a4, a5);
}

char *f76(long double a0, int a1, char *a2) { return qgcvt(a0, a1, a2); }

void f77(void *a0, size_t a1, size_t a2,
         int (*a3)(const void *, const void *)) {
  return qsort(a0, a1, a2, a3);
}

void f78(void *a0, size_t a1, size_t a2,
         int (*a3)(const void *, const void *, void *), void *a4) {
  return qsort_r(a0, a1, a2, a3, a4);
}

using t46 = quad_t;

void f79(int a0) { return quick_exit(a0); }

int f80() { return rand(); }

int f81(unsigned int *a0) { return rand_r(a0); }

long f82() { return random(); }

using t47 = struct random_data;

int f83(struct random_data *__restrict a0, int32_t *__restrict a1) {
  return random_r(a0, a1);
}

void *f84(void *a0, size_t a1) { return realloc(a0, a1); }

void *f85(void *a0, size_t a1, size_t a2) { return reallocarray(a0, a1, a2); }

char *f86(const char *__restrict a0, char *__restrict a1) {
  return realpath(a0, a1);
}

using t48 = register_t;

int f87(const char *a0) { return rpmatch(a0); }

char *f88(const char *a0) { return secure_getenv(a0); }

unsigned short *f89(unsigned short a0[3]) { return seed48(a0); }

int f90(unsigned short a0[3], struct drand48_data *a1) {
  return seed48_r(a0, a1);
}

int f91(int a0, fd_set *__restrict a1, fd_set *__restrict a2,
        fd_set *__restrict a3, struct timeval *__restrict a4) {
  return select(a0, a1, a2, a3, a4);
}

int f92(const char *a0, const char *a1, int a2) { return setenv(a0, a1, a2); }

char *f93(char *a0) { return setstate(a0); }

int f94(char *__restrict a0, struct random_data *__restrict a1) {
  return setstate_r(a0, a1);
}

using t49 = sigset_t;

using t50 = size_t;

void f95(unsigned int a0) { return srand(a0); }

void f96(long a0) { return srand48(a0); }

int f97(long a0, struct drand48_data *a1) { return srand48_r(a0, a1); }

void f98(unsigned int a0) { return srandom(a0); }

int f99(unsigned int a0, struct random_data *a1) { return srandom_r(a0, a1); }

using t51 = ssize_t;

std::ldiv_t f100(long a0, long a1) { return std::div(a0, a1); }

int f101(char *a0, size_t a1, const char *a2, double a3) {
  return strfromd(a0, a1, a2, a3);
}

int f102(char *a0, size_t a1, const char *a2, float a3) {
  return strfromf(a0, a1, a2, a3);
}

int f103(char *a0, size_t a1, const char *a2, float a3) {
  return strfromf32(a0, a1, a2, a3);
}

int f104(char *a0, size_t a1, const char *a2, double a3) {
  return strfromf32x(a0, a1, a2, a3);
}

int f105(char *a0, size_t a1, const char *a2, double a3) {
  return strfromf64(a0, a1, a2, a3);
}

int f106(char *a0, size_t a1, const char *a2, long double a3) {
  return strfromf64x(a0, a1, a2, a3);
}

int f107(char *a0, size_t a1, const char *a2, long double a3) {
  return strfroml(a0, a1, a2, a3);
}

double f108(const char *__restrict a0, char **__restrict a1) {
  return strtod(a0, a1);
}

double f109(const char *__restrict a0, char **__restrict a1, locale_t a2) {
  return strtod_l(a0, a1, a2);
}

float f110(const char *__restrict a0, char **__restrict a1) {
  return strtof(a0, a1);
}

float f111(const char *__restrict a0, char **__restrict a1) {
  return strtof32(a0, a1);
}

float f112(const char *__restrict a0, char **__restrict a1, locale_t a2) {
  return strtof32_l(a0, a1, a2);
}

double f113(const char *__restrict a0, char **__restrict a1) {
  return strtof32x(a0, a1);
}

double f114(const char *__restrict a0, char **__restrict a1, locale_t a2) {
  return strtof32x_l(a0, a1, a2);
}

double f115(const char *__restrict a0, char **__restrict a1) {
  return strtof64(a0, a1);
}

double f116(const char *__restrict a0, char **__restrict a1, locale_t a2) {
  return strtof64_l(a0, a1, a2);
}

long double f117(const char *__restrict a0, char **__restrict a1) {
  return strtof64x(a0, a1);
}

long double f118(const char *__restrict a0, char **__restrict a1, locale_t a2) {
  return strtof64x_l(a0, a1, a2);
}

float f119(const char *__restrict a0, char **__restrict a1, locale_t a2) {
  return strtof_l(a0, a1, a2);
}

long f120(const char *__restrict a0, char **__restrict a1, int a2) {
  return strtol(a0, a1, a2);
}

long f121(const char *__restrict a0, char **__restrict a1, int a2,
          locale_t a3) {
  return strtol_l(a0, a1, a2, a3);
}

long double f122(const char *__restrict a0, char **__restrict a1) {
  return strtold(a0, a1);
}

long double f123(const char *__restrict a0, char **__restrict a1, locale_t a2) {
  return strtold_l(a0, a1, a2);
}

long long f124(const char *__restrict a0, char **__restrict a1, int a2) {
  return strtoll(a0, a1, a2);
}

long long f125(const char *__restrict a0, char **__restrict a1, int a2,
               locale_t a3) {
  return strtoll_l(a0, a1, a2, a3);
}

long long f126(const char *__restrict a0, char **__restrict a1, int a2) {
  return strtoq(a0, a1, a2);
}

unsigned long f127(const char *__restrict a0, char **__restrict a1, int a2) {
  return strtoul(a0, a1, a2);
}

unsigned long f128(const char *__restrict a0, char **__restrict a1, int a2,
                   locale_t a3) {
  return strtoul_l(a0, a1, a2, a3);
}

unsigned long long f129(const char *__restrict a0, char **__restrict a1,
                        int a2) {
  return strtoull(a0, a1, a2);
}

unsigned long long f130(const char *__restrict a0, char **__restrict a1, int a2,
                        locale_t a3) {
  return strtoull_l(a0, a1, a2, a3);
}

unsigned long long f131(const char *__restrict a0, char **__restrict a1,
                        int a2) {
  return strtouq(a0, a1, a2);
}

using t52 = suseconds_t;

int f132(const char *a0) { return system(a0); }

using t53 = timer_t;

using t54 = struct timespec;

using t55 = struct timeval;

using t56 = u_char;

using t57 = u_int;

using t58 = u_int16_t;

using t59 = u_int32_t;

using t60 = u_int64_t;

using t61 = u_int8_t;

using t62 = u_long;

using t63 = u_quad_t;

using t64 = u_short;

using t65 = uid_t;

using t66 = uint;

using t67 = ulong;

int f133(int a0) { return unlockpt(a0); }

int f134(const char *a0) { return unsetenv(a0); }

using t68 = useconds_t;

using t69 = ushort;

void *f135(size_t a0) { return valloc(a0); }

size_t f136(char *__restrict a0, const wchar_t *__restrict a1, size_t a2) {
  return wcstombs(a0, a1, a2);
}

int f137(char *a0, wchar_t a1) { return wctomb(a0, a1); }
