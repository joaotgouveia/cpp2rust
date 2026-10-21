#include "../common/init.h"
#include "../common/traits.h"
#include <cstring>
#include <stdlib.h>
#include <utility>

const char *f1(const char *a0) { return basename(a0); }

char *f2(char *a0) { return basename(a0); }

int f3(const void *a0, const void *a1, size_t a2) { return bcmp(a0, a1, a2); }

void f4(const void *a0, void *a1, size_t a2) { return bcopy(a0, a1, a2); }

void f5(void *a0, size_t a1) { return bzero(a0, a1); }

void f6(void *a0, size_t a1) { return explicit_bzero(a0, a1); }

int f7(int a0) { return ffs(a0); }

int f8(long a0) { return ffsl(a0); }

int f9(long long a0) { return ffsll(a0); }

char *f10(const char *a0, int a1) { return index(a0, a1); }

void *f11(void *__restrict a0, const void *__restrict a1, int a2, size_t a3) {
  return memccpy(a0, a1, a2, a3);
}

const void *f12(const void *a0, int a1, size_t a2) {
  return memchr(a0, a1, a2);
}

void *f13(void *a0, int a1, size_t a2) { return memchr(a0, a1, a2); }

int f14(const void *a0, const void *a1, size_t a2) {
  return memcmp(a0, a1, a2);
}

void *f15(void *__restrict a0, const void *__restrict a1, size_t a2) {
  return memcpy(a0, a1, a2);
}

void *f16(void *a0, size_t a1) { return memfrob(a0, a1); }

void *f17(const void *a0, size_t a1, const void *a2, size_t a3) {
  return memmem(a0, a1, a2, a3);
}

void *f18(void *a0, const void *a1, size_t a2) { return memmove(a0, a1, a2); }

void *f19(void *__restrict a0, const void *__restrict a1, size_t a2) {
  return mempcpy(a0, a1, a2);
}

const void *f20(const void *a0, int a1, size_t a2) {
  return memrchr(a0, a1, a2);
}

void *f21(void *a0, int a1, size_t a2) { return memrchr(a0, a1, a2); }

void *f22(void *a0, int a1, size_t a2) { return memset(a0, a1, a2); }

const void *f23(const void *a0, int a1) { return rawmemchr(a0, a1); }

void *f24(void *a0, int a1) { return rawmemchr(a0, a1); }

char *f25(const char *a0, int a1) { return rindex(a0, a1); }

const char *f26(int a0) { return sigabbrev_np(a0); }

const char *f27(int a0) { return sigdescr_np(a0); }

char *f28(char *__restrict a0, const char *__restrict a1) {
  return stpcpy(a0, a1);
}

char *f29(char *__restrict a0, const char *__restrict a1, size_t a2) {
  return stpncpy(a0, a1, a2);
}

int f30(const char *a0, const char *a1) { return strcasecmp(a0, a1); }

int f31(const char *a0, const char *a1, locale_t a2) {
  return strcasecmp_l(a0, a1, a2);
}

const char *f32(const char *a0, const char *a1) { return strcasestr(a0, a1); }

char *f33(char *a0, const char *a1) { return strcasestr(a0, a1); }

char *f34(char *__restrict a0, const char *__restrict a1) {
  return strcat(a0, a1);
}

const char *f35(const char *a0, int a1) { return strchr(a0, a1); }

char *f36(char *a0, int a1) { return strchr(a0, a1); }

const char *f37(const char *a0, int a1) { return strchrnul(a0, a1); }

char *f38(char *a0, int a1) { return strchrnul(a0, a1); }

int f39(const char *a0, const char *a1) { return strcmp(a0, a1); }

int f40(const char *a0, const char *a1) { return strcoll(a0, a1); }

int f41(const char *a0, const char *a1, locale_t a2) {
  return strcoll_l(a0, a1, a2);
}

char *f42(char *__restrict a0, const char *__restrict a1) {
  return strcpy(a0, a1);
}

size_t f43(const char *a0, const char *a1) { return strcspn(a0, a1); }

char *f44(const char *a0) { return strdup(a0); }

char *f45(int a0) { return strerror(a0); }

char *f46(int a0, locale_t a1) { return strerror_l(a0, a1); }

char *f47(int a0, char *a1, size_t a2) { return strerror_r(a0, a1, a2); }

const char *f48(int a0) { return strerrordesc_np(a0); }

const char *f49(int a0) { return strerrorname_np(a0); }

char *f50(char *a0) { return strfry(a0); }

size_t f51(char *__restrict a0, const char *__restrict a1, size_t a2) {
  return strlcat(a0, a1, a2);
}

size_t f52(char *__restrict a0, const char *__restrict a1, size_t a2) {
  return strlcpy(a0, a1, a2);
}

size_t f53(const char *a0) { return strlen(a0); }

int f54(const char *a0, const char *a1, size_t a2) {
  return strncasecmp(a0, a1, a2);
}

int f55(const char *a0, const char *a1, size_t a2, locale_t a3) {
  return strncasecmp_l(a0, a1, a2, a3);
}

char *f56(char *__restrict a0, const char *__restrict a1, size_t a2) {
  return strncat(a0, a1, a2);
}

int f57(const char *a0, const char *a1, size_t a2) {
  return strncmp(a0, a1, a2);
}

char *f58(char *__restrict a0, const char *__restrict a1, size_t a2) {
  return strncpy(a0, a1, a2);
}

char *f59(const char *a0, size_t a1) { return strndup(a0, a1); }

size_t f60(const char *a0, size_t a1) { return strnlen(a0, a1); }

const char *f61(const char *a0, const char *a1) { return strpbrk(a0, a1); }

char *f62(char *a0, const char *a1) { return strpbrk(a0, a1); }

const char *f63(const char *a0, int a1) { return strrchr(a0, a1); }

char *f64(char *a0, int a1) { return strrchr(a0, a1); }

char *f65(char **__restrict a0, const char *__restrict a1) {
  return strsep(a0, a1);
}

char *f66(int a0) { return strsignal(a0); }

size_t f67(const char *a0, const char *a1) { return strspn(a0, a1); }

const char *f68(const char *a0, const char *a1) { return strstr(a0, a1); }

char *f69(char *a0, const char *a1) { return strstr(a0, a1); }

char *f70(char *__restrict a0, const char *__restrict a1) {
  return strtok(a0, a1);
}

char *f71(char *__restrict a0, const char *__restrict a1,
          char **__restrict a2) {
  return strtok_r(a0, a1, a2);
}

int f72(const char *a0, const char *a1) { return strverscmp(a0, a1); }

size_t f73(char *__restrict a0, const char *__restrict a1, size_t a2) {
  return strxfrm(a0, a1, a2);
}

size_t f74(char *a0, const char *a1, size_t a2, locale_t a3) {
  return strxfrm_l(a0, a1, a2, a3);
}
