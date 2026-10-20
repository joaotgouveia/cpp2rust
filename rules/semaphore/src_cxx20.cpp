#include "../common/init.h"
#include "../common/traits.h"
#include <semaphore>
#include <stdlib.h>
#include <utility>

int f1(const struct timeval *a0, struct timeval *a1) { return adjtime(a0, a1); }

int f2(int a0, const struct timeval a1[2]) { return futimes(a0, a1); }

int f3(int a0, const char *a1, const struct timeval a2[2]) {
  return futimesat(a0, a1, a2);
}

int f4(int a0, struct itimerval *a1) { return getitimer(a0, a1); }

int f5(struct timeval *__restrict a0, void *__restrict a1) {
  return gettimeofday(a0, a1);
}

using t1 = struct itimerval;

int f6(const char *a0, const struct timeval a1[2]) { return lutimes(a0, a1); }

int f7(sem_t *__restrict a0, clockid_t a1,
       const struct timespec *__restrict a2) {
  return sem_clockwait(a0, a1, a2);
}

int f8(sem_t *a0) { return sem_close(a0); }

int f9(sem_t *a0) { return sem_destroy(a0); }

int f10(sem_t *__restrict a0, int *__restrict a1) {
  return sem_getvalue(a0, a1);
}

int f11(sem_t *a0, int a1, unsigned int a2) { return sem_init(a0, a1, a2); }

template <typename... Args> sem_t *f12(const char *a0, int a1, Args... args) {
  return sem_open(a0, a1, args...);
}

int f13(sem_t *a0) { return sem_post(a0); }

using t2 = sem_t;

int f14(sem_t *__restrict a0, const struct timespec *__restrict a1) {
  return sem_timedwait(a0, a1);
}

int f15(sem_t *a0) { return sem_trywait(a0); }

int f16(const char *a0) { return sem_unlink(a0); }

int f17(sem_t *a0) { return sem_wait(a0); }

int f18(int a0, const struct itimerval *__restrict a1,
        struct itimerval *__restrict a2) {
  return setitimer(a0, a1, a2);
}

int f19(const struct timeval *a0, const struct timezone *a1) {
  return settimeofday(a0, a1);
}

using t3 = std::binary_semaphore;

using t4 = struct timezone;

int f20(const char *a0, const struct timeval a1[2]) { return utimes(a0, a1); }
