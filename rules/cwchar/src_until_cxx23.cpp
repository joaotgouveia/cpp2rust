#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <cwchar>
#include <stdlib.h>
#include <utility>

wint_t f88(FILE *a0) { return fgetwc(a0); }

wint_t f89(FILE *a0) { return fgetwc_unlocked(a0); }

wchar_t *f90(wchar_t *__restrict a0, int a1, FILE *__restrict a2) {
  return fgetws(a0, a1, a2);
}

wchar_t *f91(wchar_t *__restrict a0, int a1, FILE *__restrict a2) {
  return fgetws_unlocked(a0, a1, a2);
}

wint_t f92(wchar_t a0, FILE *a1) { return fputwc(a0, a1); }

wint_t f93(wchar_t a0, FILE *a1) { return fputwc_unlocked(a0, a1); }

int f94(const wchar_t *__restrict a0, FILE *__restrict a1) {
  return fputws(a0, a1);
}

int f95(const wchar_t *__restrict a0, FILE *__restrict a1) {
  return fputws_unlocked(a0, a1);
}

int f96(FILE *a0, int a1) { return fwide(a0, a1); }

template <typename... Args>
int f97(FILE *__restrict a0, const wchar_t *__restrict a1, Args... args) {
  return fwprintf(a0, a1, args...);
}

template <typename... Args>
int f98(FILE *__restrict a0, const wchar_t *__restrict a1, Args... args) {
  return fwscanf(a0, a1, args...);
}

wint_t f99(FILE *a0) { return getwc(a0); }

wint_t f100(FILE *a0) { return getwc_unlocked(a0); }

FILE *f101(wchar_t **a0, size_t *a1) { return open_wmemstream(a0, a1); }

wint_t f102(wchar_t a0, FILE *a1) { return putwc(a0, a1); }

wint_t f103(wchar_t a0, FILE *a1) { return putwc_unlocked(a0, a1); }

wint_t f104(FILE *a0) { return fgetwc(a0); }

wchar_t *f105(wchar_t *__restrict a0, int a1, FILE *__restrict a2) {
  return fgetws(a0, a1, a2);
}

wint_t f106(wchar_t a0, FILE *a1) { return fputwc(a0, a1); }

int f107(const wchar_t *__restrict a0, FILE *__restrict a1) {
  return fputws(a0, a1);
}

int f108(FILE *a0, int a1) { return fwide(a0, a1); }

template <typename... Args>
int f109(FILE *__restrict a0, const wchar_t *__restrict a1, Args... args) {
  return fwprintf(a0, a1, args...);
}

template <typename... Args>
int f110(FILE *__restrict a0, const wchar_t *__restrict a1, Args... args) {
  return fwscanf(a0, a1, args...);
}

wint_t f111(FILE *a0) { return getwc(a0); }

wint_t f112(wchar_t a0, FILE *a1) { return putwc(a0, a1); }

wint_t f113(wint_t a0, FILE *a1) { return ungetwc(a0, a1); }

int f114(FILE *__restrict a0, const wchar_t *__restrict a1, va_list a2) {
  return vfwprintf(a0, a1, a2);
}

int f115(FILE *__restrict a0, const wchar_t *__restrict a1, va_list a2) {
  return vfwscanf(a0, a1, a2);
}

int f116(wchar_t *__restrict a0, size_t a1, const wchar_t *__restrict a2,
         va_list a3) {
  return vswprintf(a0, a1, a2, a3);
}

int f117(const wchar_t *__restrict a0, const wchar_t *__restrict a1,
         va_list a2) {
  return vswscanf(a0, a1, a2);
}

int f118(const wchar_t *__restrict a0, va_list a1) { return vwprintf(a0, a1); }

int f119(const wchar_t *__restrict a0, va_list a1) { return vwscanf(a0, a1); }

wint_t f120(wint_t a0, FILE *a1) { return ungetwc(a0, a1); }

int f121(FILE *__restrict a0, const wchar_t *__restrict a1, va_list a2) {
  return vfwprintf(a0, a1, a2);
}

int f122(FILE *__restrict a0, const wchar_t *__restrict a1, va_list a2) {
  return vfwscanf(a0, a1, a2);
}

int f123(wchar_t *__restrict a0, size_t a1, const wchar_t *__restrict a2,
         va_list a3) {
  return vswprintf(a0, a1, a2, a3);
}

int f124(const wchar_t *__restrict a0, const wchar_t *__restrict a1,
         va_list a2) {
  return vswscanf(a0, a1, a2);
}

int f125(const wchar_t *__restrict a0, va_list a1) { return vwprintf(a0, a1); }

int f126(const wchar_t *__restrict a0, va_list a1) { return vwscanf(a0, a1); }
