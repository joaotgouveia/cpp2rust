#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <cwchar>
#include <stdlib.h>
#include <utility>

using t1 = FILE;

wint_t f1(int a0) { return btowc(a0); }

wint_t f2() { return getwchar(); }

wint_t f3() { return getwchar_unlocked(); }

size_t f4(const char *__restrict a0, size_t a1, mbstate_t *__restrict a2) {
  return mbrlen(a0, a1, a2);
}

size_t f5(wchar_t *__restrict a0, const char *__restrict a1, size_t a2,
          mbstate_t *__restrict a3) {
  return mbrtowc(a0, a1, a2, a3);
}

int f6(const mbstate_t *a0) { return mbsinit(a0); }

size_t f7(wchar_t *__restrict a0, const char **__restrict a1, size_t a2,
          size_t a3, mbstate_t *__restrict a4) {
  return mbsnrtowcs(a0, a1, a2, a3, a4);
}

size_t f8(wchar_t *__restrict a0, const char **__restrict a1, size_t a2,
          mbstate_t *__restrict a3) {
  return mbsrtowcs(a0, a1, a2, a3);
}

using t2 = mbstate_t;

wint_t f9(wchar_t a0) { return putwchar(a0); }

wint_t f10(wchar_t a0) { return putwchar_unlocked(a0); }

template <typename... Args>
int f11(wchar_t *__restrict a0, size_t a1, const wchar_t *__restrict a2,
        Args... args) {
  return swprintf(a0, a1, a2, args...);
}

template <typename... Args>
int f12(const wchar_t *__restrict a0, const wchar_t *__restrict a1,
        Args... args) {
  return swscanf(a0, a1, args...);
}

size_t f13(char *__restrict a0, wchar_t a1, mbstate_t *__restrict a2) {
  return wcrtomb(a0, a1, a2);
}

wchar_t *f14(wchar_t *__restrict a0, const wchar_t *__restrict a1) {
  return wcscat(a0, a1);
}

wchar_t *f15(wchar_t *a0, wchar_t a1) { return std::wcschr(a0, a1); }

wchar_t *f16(const wchar_t *a0, wchar_t a1) { return wcschr(a0, a1); }

int f17(const wchar_t *a0, const wchar_t *a1) { return wcscmp(a0, a1); }

int f18(const wchar_t *a0, const wchar_t *a1) { return wcscoll(a0, a1); }

wchar_t *f19(wchar_t *__restrict a0, const wchar_t *__restrict a1) {
  return wcscpy(a0, a1);
}

size_t f20(const wchar_t *a0, const wchar_t *a1) { return wcscspn(a0, a1); }

size_t f21(wchar_t *__restrict a0, size_t a1, const wchar_t *__restrict a2,
           const struct tm *__restrict a3) {
  return wcsftime(a0, a1, a2, a3);
}

size_t f22(const wchar_t *a0) { return wcslen(a0); }

wchar_t *f23(wchar_t *__restrict a0, const wchar_t *__restrict a1, size_t a2) {
  return wcsncat(a0, a1, a2);
}

int f24(const wchar_t *a0, const wchar_t *a1, size_t a2) {
  return wcsncmp(a0, a1, a2);
}

wchar_t *f25(wchar_t *__restrict a0, const wchar_t *__restrict a1, size_t a2) {
  return wcsncpy(a0, a1, a2);
}

wchar_t *f26(wchar_t *a0, const wchar_t *a1) { return std::wcspbrk(a0, a1); }

wchar_t *f27(const wchar_t *a0, const wchar_t *a1) { return wcspbrk(a0, a1); }

wchar_t *f28(wchar_t *a0, wchar_t a1) { return std::wcsrchr(a0, a1); }

wchar_t *f29(const wchar_t *a0, wchar_t a1) { return wcsrchr(a0, a1); }

size_t f30(char *__restrict a0, const wchar_t **__restrict a1, size_t a2,
           mbstate_t *__restrict a3) {
  return wcsrtombs(a0, a1, a2, a3);
}

size_t f31(const wchar_t *a0, const wchar_t *a1) { return wcsspn(a0, a1); }

wchar_t *f32(wchar_t *a0, const wchar_t *a1) { return std::wcsstr(a0, a1); }

wchar_t *f33(const wchar_t *a0, const wchar_t *a1) { return wcsstr(a0, a1); }

double f34(const wchar_t *__restrict a0, wchar_t **__restrict a1) {
  return wcstod(a0, a1);
}

float f35(const wchar_t *__restrict a0, wchar_t **__restrict a1) {
  return wcstof(a0, a1);
}

wchar_t *f36(wchar_t *__restrict a0, const wchar_t *__restrict a1,
             wchar_t **__restrict a2) {
  return wcstok(a0, a1, a2);
}

long f37(const wchar_t *__restrict a0, wchar_t **__restrict a1, int a2) {
  return wcstol(a0, a1, a2);
}

long double f38(const wchar_t *__restrict a0, wchar_t **__restrict a1) {
  return wcstold(a0, a1);
}

long long f39(const wchar_t *__restrict a0, wchar_t **__restrict a1, int a2) {
  return wcstoll(a0, a1, a2);
}

unsigned long f40(const wchar_t *__restrict a0, wchar_t **__restrict a1,
                  int a2) {
  return wcstoul(a0, a1, a2);
}

unsigned long long f41(const wchar_t *__restrict a0, wchar_t **__restrict a1,
                       int a2) {
  return wcstoull(a0, a1, a2);
}

size_t f42(wchar_t *__restrict a0, const wchar_t *__restrict a1, size_t a2) {
  return wcsxfrm(a0, a1, a2);
}

int f43(wint_t a0) { return wctob(a0); }

using t3 = wint_t;

wchar_t *f44(wchar_t *a0, wchar_t a1, std::size_t a2) {
  return std::wmemchr(a0, a1, a2);
}

wchar_t *f45(const wchar_t *a0, wchar_t a1, size_t a2) {
  return wmemchr(a0, a1, a2);
}

int f46(const wchar_t *a0, const wchar_t *a1, size_t a2) {
  return wmemcmp(a0, a1, a2);
}

wchar_t *f47(wchar_t *__restrict a0, const wchar_t *__restrict a1, size_t a2) {
  return wmemcpy(a0, a1, a2);
}

wchar_t *f48(wchar_t *a0, const wchar_t *a1, size_t a2) {
  return wmemmove(a0, a1, a2);
}

wchar_t *f49(wchar_t *a0, wchar_t a1, size_t a2) { return wmemset(a0, a1, a2); }

template <typename... Args>
int f50(const wchar_t *__restrict a0, Args... args) {
  return wprintf(a0, args...);
}

template <typename... Args>
int f51(const wchar_t *__restrict a0, Args... args) {
  return wscanf(a0, args...);
}

using t4 = va_list;

wchar_t *f52(wchar_t *__restrict a0, const wchar_t *__restrict a1) {
  return wcpcpy(a0, a1);
}

wchar_t *f53(wchar_t *__restrict a0, const wchar_t *__restrict a1, size_t a2) {
  return wcpncpy(a0, a1, a2);
}

int f54(const wchar_t *a0, const wchar_t *a1) { return wcscasecmp(a0, a1); }

int f55(const wchar_t *a0, const wchar_t *a1, locale_t a2) {
  return wcscasecmp_l(a0, a1, a2);
}

wchar_t *f56(const wchar_t *a0, wchar_t a1) { return wcschrnul(a0, a1); }

int f57(const wchar_t *a0, const wchar_t *a1, locale_t a2) {
  return wcscoll_l(a0, a1, a2);
}

wchar_t *f58(const wchar_t *a0) { return wcsdup(a0); }

size_t f59(wchar_t *__restrict a0, size_t a1, const wchar_t *__restrict a2,
           const struct tm *__restrict a3, locale_t a4) {
  return wcsftime_l(a0, a1, a2, a3, a4);
}

size_t f60(wchar_t *__restrict a0, const wchar_t *__restrict a1, size_t a2) {
  return wcslcat(a0, a1, a2);
}

size_t f61(wchar_t *__restrict a0, const wchar_t *__restrict a1, size_t a2) {
  return wcslcpy(a0, a1, a2);
}

int f62(const wchar_t *a0, const wchar_t *a1, size_t a2) {
  return wcsncasecmp(a0, a1, a2);
}

int f63(const wchar_t *a0, const wchar_t *a1, size_t a2, locale_t a3) {
  return wcsncasecmp_l(a0, a1, a2, a3);
}

size_t f64(const wchar_t *a0, size_t a1) { return wcsnlen(a0, a1); }

size_t f65(char *__restrict a0, const wchar_t **__restrict a1, size_t a2,
           size_t a3, mbstate_t *__restrict a4) {
  return wcsnrtombs(a0, a1, a2, a3, a4);
}

double f66(const wchar_t *__restrict a0, wchar_t **__restrict a1, locale_t a2) {
  return wcstod_l(a0, a1, a2);
}

float f67(const wchar_t *__restrict a0, wchar_t **__restrict a1) {
  return wcstof32(a0, a1);
}

float f68(const wchar_t *__restrict a0, wchar_t **__restrict a1, locale_t a2) {
  return wcstof32_l(a0, a1, a2);
}

double f69(const wchar_t *__restrict a0, wchar_t **__restrict a1) {
  return wcstof32x(a0, a1);
}

double f70(const wchar_t *__restrict a0, wchar_t **__restrict a1, locale_t a2) {
  return wcstof32x_l(a0, a1, a2);
}

double f71(const wchar_t *__restrict a0, wchar_t **__restrict a1) {
  return wcstof64(a0, a1);
}

double f72(const wchar_t *__restrict a0, wchar_t **__restrict a1, locale_t a2) {
  return wcstof64_l(a0, a1, a2);
}

long double f73(const wchar_t *__restrict a0, wchar_t **__restrict a1) {
  return wcstof64x(a0, a1);
}

long double f74(const wchar_t *__restrict a0, wchar_t **__restrict a1,
                locale_t a2) {
  return wcstof64x_l(a0, a1, a2);
}

float f75(const wchar_t *__restrict a0, wchar_t **__restrict a1, locale_t a2) {
  return wcstof_l(a0, a1, a2);
}

long f76(const wchar_t *__restrict a0, wchar_t **__restrict a1, int a2,
         locale_t a3) {
  return wcstol_l(a0, a1, a2, a3);
}

long double f77(const wchar_t *__restrict a0, wchar_t **__restrict a1,
                locale_t a2) {
  return wcstold_l(a0, a1, a2);
}

long long f78(const wchar_t *__restrict a0, wchar_t **__restrict a1, int a2,
              locale_t a3) {
  return wcstoll_l(a0, a1, a2, a3);
}

long long f79(const wchar_t *__restrict a0, wchar_t **__restrict a1, int a2) {
  return wcstoq(a0, a1, a2);
}

unsigned long f80(const wchar_t *__restrict a0, wchar_t **__restrict a1, int a2,
                  locale_t a3) {
  return wcstoul_l(a0, a1, a2, a3);
}

unsigned long long f81(const wchar_t *__restrict a0, wchar_t **__restrict a1,
                       int a2, locale_t a3) {
  return wcstoull_l(a0, a1, a2, a3);
}

unsigned long long f82(const wchar_t *__restrict a0, wchar_t **__restrict a1,
                       int a2) {
  return wcstouq(a0, a1, a2);
}

wchar_t *f83(const wchar_t *a0, const wchar_t *a1) { return wcswcs(a0, a1); }

int f84(const wchar_t *a0, size_t a1) { return wcswidth(a0, a1); }

size_t f85(wchar_t *a0, const wchar_t *a1, size_t a2, locale_t a3) {
  return wcsxfrm_l(a0, a1, a2, a3);
}

int f86(wchar_t a0) { return wcwidth(a0); }

wchar_t *f87(wchar_t *__restrict a0, const wchar_t *__restrict a1, size_t a2) {
  return wmempcpy(a0, a1, a2);
}
