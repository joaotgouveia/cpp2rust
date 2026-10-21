#include "../common/init.h"
#include "../common/traits.h"
#include <cstdio>
#include <cwchar>
#include <stdlib.h>
#include <utility>

using t1 = FILE;

template <typename... Args>
int f1(char **__restrict a0, const char *__restrict a1, Args... args) {
  return asprintf(a0, a1, args...);
}

void f2(FILE *a0) { return clearerr(a0); }

void f3(FILE *a0) { return clearerr_unlocked(a0); }

using t2 = cookie_close_function_t;

using t3 = cookie_io_functions_t;

using t4 = cookie_read_function_t;

using t5 = cookie_seek_function_t;

using t6 = cookie_write_function_t;

char *f4(char *a0) { return ctermid(a0); }

char *f5(char *a0) { return cuserid(a0); }

template <typename... Args>
int f6(int a0, const char *__restrict a1, Args... args) {
  return dprintf(a0, a1, args...);
}

int f7(FILE *a0) { return fclose(a0); }

int f8() { return fcloseall(); }

FILE *f9(int a0, const char *a1) { return fdopen(a0, a1); }

int f10(FILE *a0) { return feof(a0); }

int f11(FILE *a0) { return feof_unlocked(a0); }

int f12(FILE *a0) { return ferror(a0); }

int f13(FILE *a0) { return ferror_unlocked(a0); }

int f14(FILE *a0) { return fflush(a0); }

int f15(FILE *a0) { return fflush_unlocked(a0); }

int f16(FILE *a0) { return fgetc(a0); }

int f17(FILE *a0) { return fgetc_unlocked(a0); }

int f18(FILE *__restrict a0, fpos_t *__restrict a1) { return fgetpos(a0, a1); }

int f19(FILE *__restrict a0, fpos64_t *__restrict a1) {
  return fgetpos64(a0, a1);
}

char *f20(char *__restrict a0, int a1, FILE *__restrict a2) {
  return fgets(a0, a1, a2);
}

char *f21(char *__restrict a0, int a1, FILE *__restrict a2) {
  return fgets_unlocked(a0, a1, a2);
}

int f22(FILE *a0) { return fileno(a0); }

int f23(FILE *a0) { return fileno_unlocked(a0); }

void f24(FILE *a0) { return flockfile(a0); }

FILE *f25(void *a0, size_t a1, const char *a2) { return fmemopen(a0, a1, a2); }

FILE *f26(const char *__restrict a0, const char *__restrict a1) {
  return fopen(a0, a1);
}

FILE *f27(const char *__restrict a0, const char *__restrict a1) {
  return fopen64(a0, a1);
}

FILE *f28(void *__restrict a0, const char *__restrict a1,
          cookie_io_functions_t a2) {
  return fopencookie(a0, a1, std::move(a2));
}

using t7 = fpos64_t;

using t8 = fpos_t;

template <typename... Args>
int f29(FILE *__restrict a0, const char *__restrict a1, Args... args) {
  return fprintf(a0, a1, args...);
}

int f30(int a0, FILE *a1) { return fputc(a0, a1); }

int f31(int a0, FILE *a1) { return fputc_unlocked(a0, a1); }

int f32(const char *__restrict a0, FILE *__restrict a1) {
  return fputs(a0, a1);
}

int f33(const char *__restrict a0, FILE *__restrict a1) {
  return fputs_unlocked(a0, a1);
}

size_t f34(void *__restrict a0, size_t a1, size_t a2, FILE *__restrict a3) {
  return fread(a0, a1, a2, a3);
}

size_t f35(void *__restrict a0, size_t a1, size_t a2, FILE *__restrict a3) {
  return fread_unlocked(a0, a1, a2, a3);
}

FILE *f36(const char *__restrict a0, const char *__restrict a1,
          FILE *__restrict a2) {
  return freopen(a0, a1, a2);
}

FILE *f37(const char *__restrict a0, const char *__restrict a1,
          FILE *__restrict a2) {
  return freopen64(a0, a1, a2);
}

template <typename... Args>
int f38(FILE *__restrict a0, const char *__restrict a1, Args... args) {
  return fscanf(a0, a1, args...);
}

int f39(FILE *a0, long a1, int a2) { return fseek(a0, a1, a2); }

int f40(FILE *a0, long a1, int a2) { return fseeko(a0, a1, a2); }

int f41(FILE *a0, long a1, int a2) { return fseeko64(a0, a1, a2); }

int f42(FILE *a0, const fpos_t *a1) { return fsetpos(a0, a1); }

int f43(FILE *a0, const fpos64_t *a1) { return fsetpos64(a0, a1); }

long f44(FILE *a0) { return ftell(a0); }

long f45(FILE *a0) { return ftello(a0); }

long f46(FILE *a0) { return ftello64(a0); }

int f47(FILE *a0) { return ftrylockfile(a0); }

void f48(FILE *a0) { return funlockfile(a0); }

size_t f49(const void *__restrict a0, size_t a1, size_t a2,
           FILE *__restrict a3) {
  return fwrite(a0, a1, a2, a3);
}

size_t f50(const void *__restrict a0, size_t a1, size_t a2,
           FILE *__restrict a3) {
  return fwrite_unlocked(a0, a1, a2, a3);
}

int f51(FILE *a0) { return getc(a0); }

int f52(FILE *a0) { return getc_unlocked(a0); }

int f53() { return getchar(); }

int f54() { return getchar_unlocked(); }

long f55(char **__restrict a0, size_t *__restrict a1, int a2,
         FILE *__restrict a3) {
  return getdelim(a0, a1, a2, a3);
}

long f56(char **__restrict a0, size_t *__restrict a1, FILE *__restrict a2) {
  return getline(a0, a1, a2);
}

int f57(FILE *a0) { return getw(a0); }

template <typename... Args>
int f58(struct obstack *__restrict a0, const char *__restrict a1,
        Args... args) {
  return obstack_printf(a0, a1, args...);
}

FILE *f59(char **a0, size_t *a1) { return open_memstream(a0, a1); }

int f60(FILE *a0) { return pclose(a0); }

void f61(const char *a0) { return perror(a0); }

FILE *f62(const char *a0, const char *a1) { return popen(a0, a1); }

template <typename... Args> int f63(const char *__restrict a0, Args... args) {
  return printf(a0, args...);
}

int f64(int a0, FILE *a1) { return putc(a0, a1); }

int f65(int a0, FILE *a1) { return putc_unlocked(a0, a1); }

int f66(int a0) { return putchar(a0); }

int f67(int a0) { return putchar_unlocked(a0); }

int f68(const char *a0) { return puts(a0); }

int f69(int a0, FILE *a1) { return putw(a0, a1); }

int f70(const char *a0) { return remove(a0); }

int f71(const char *a0, const char *a1) { return rename(a0, a1); }

int f72(int a0, const char *a1, int a2, const char *a3) {
  return renameat(a0, a1, a2, a3);
}

int f73(int a0, const char *a1, int a2, const char *a3, unsigned int a4) {
  return renameat2(a0, a1, a2, a3, a4);
}

void f74(FILE *a0) { return rewind(a0); }

template <typename... Args> int f75(const char *__restrict a0, Args... args) {
  return scanf(a0, args...);
}

void f76(FILE *__restrict a0, char *__restrict a1) { return setbuf(a0, a1); }

void f77(FILE *__restrict a0, char *__restrict a1, size_t a2) {
  return setbuffer(a0, a1, a2);
}

void f78(FILE *a0) { return setlinebuf(a0); }

int f79(FILE *__restrict a0, char *__restrict a1, int a2, size_t a3) {
  return setvbuf(a0, a1, a2, a3);
}

template <typename... Args>
int f80(char *__restrict a0, size_t a1, const char *__restrict a2,
        Args... args) {
  return snprintf(a0, a1, a2, args...);
}

template <typename... Args>
int f81(char *__restrict a0, const char *__restrict a1, Args... args) {
  return sprintf(a0, a1, args...);
}

template <typename... Args>
int f82(const char *__restrict a0, const char *__restrict a1, Args... args) {
  return sscanf(a0, a1, args...);
}

char *f83(const char *a0, const char *a1) { return tempnam(a0, a1); }

FILE *f84() { return tmpfile(); }

FILE *f85() { return tmpfile64(); }

char *f86(char a0[20]) { return tmpnam(a0); }

char *f87(char a0[20]) { return tmpnam_r(a0); }

int f88(int a0, FILE *a1) { return ungetc(a0, a1); }
