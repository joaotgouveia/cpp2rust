#include "../common/init.h"
#include "../common/traits.h"
#include <cstdio>
#include <cwchar>
#include <stdlib.h>
#include <utility>

int f89(struct obstack *__restrict a0, const char *__restrict a1, va_list a2) {
  return obstack_vprintf(a0, a1, a2);
}

int f90(FILE *__restrict a0, const char *__restrict a1, va_list a2) {
  return vfprintf(a0, a1, a2);
}

int f91(FILE *__restrict a0, const char *__restrict a1, va_list a2) {
  return vfscanf(a0, a1, a2);
}

int f92(const char *__restrict a0, va_list a1) { return vprintf(a0, a1); }

int f93(const char *__restrict a0, va_list a1) { return vscanf(a0, a1); }

int f94(char *__restrict a0, size_t a1, const char *__restrict a2, va_list a3) {
  return vsnprintf(a0, a1, a2, a3);
}

int f95(char *__restrict a0, const char *__restrict a1, va_list a2) {
  return vsprintf(a0, a1, a2);
}

int f96(const char *__restrict a0, const char *__restrict a1, va_list a2) {
  return vsscanf(a0, a1, a2);
}

int f97(char **__restrict a0, const char *__restrict a1, va_list a2) {
  return vasprintf(a0, a1, a2);
}

int f98(int a0, const char *__restrict a1, va_list a2) {
  return vdprintf(a0, a1, a2);
}

int f99(FILE *__restrict a0, const char *__restrict a1, va_list a2) {
  return vfprintf(a0, a1, a2);
}

int f100(FILE *__restrict a0, const char *__restrict a1, va_list a2) {
  return vfscanf(a0, a1, a2);
}

int f101(const char *__restrict a0, va_list a1) { return vprintf(a0, a1); }

int f102(const char *__restrict a0, va_list a1) { return vscanf(a0, a1); }

int f103(char *__restrict a0, size_t a1, const char *__restrict a2,
         va_list a3) {
  return vsnprintf(a0, a1, a2, a3);
}

int f104(char *__restrict a0, const char *__restrict a1, va_list a2) {
  return vsprintf(a0, a1, a2);
}

int f105(const char *__restrict a0, const char *__restrict a1, va_list a2) {
  return vsscanf(a0, a1, a2);
}
