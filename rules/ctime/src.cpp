#include "../common/init.h"
#include "../common/traits.h"
#include <ios>
#include <stdlib.h>
#include <utility>

char *f1(const struct tm *a0) { return asctime(a0); }

clock_t f2() { return clock(); }

using t1 = clock_t;

char *f3(const time_t *a0) { return ctime(a0); }

double f4(time_t a0, time_t a1) { return difftime(a0, a1); }

struct tm *f5(const time_t *a0) { return gmtime(a0); }

struct tm *f6(const time_t *a0) { return localtime(a0); }

time_t f7(struct tm *a0) { return mktime(a0); }

size_t f8(char *__restrict a0, size_t a1, const char *__restrict a2,
          const struct tm *__restrict a3) {
  return strftime(a0, a1, a2, a3);
}

time_t f9(time_t *a0) { return time(a0); }

using t2 = time_t;
