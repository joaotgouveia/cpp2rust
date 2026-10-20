#include "../common/init.h"
#include "../common/traits.h"
#include <cinttypes>
#include <cstdint>
#include <utility>

intmax_t f1(intmax_t a0) { return imaxabs(a0); }

imaxdiv_t f2(intmax_t a0, intmax_t a1) { return imaxdiv(a0, a1); }

using t1 = imaxdiv_t;

intmax_t f3(const char *__restrict a0, char **__restrict a1, int a2) {
  return strtoimax(a0, a1, a2);
}

uintmax_t f4(const char *__restrict a0, char **__restrict a1, int a2) {
  return strtoumax(a0, a1, a2);
}

intmax_t f5(const wchar_t *__restrict a0, wchar_t **__restrict a1, int a2) {
  return wcstoimax(a0, a1, a2);
}

uintmax_t f6(const wchar_t *__restrict a0, wchar_t **__restrict a1, int a2) {
  return wcstoumax(a0, a1, a2);
}
