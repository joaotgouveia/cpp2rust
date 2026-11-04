// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#undef _GNU_SOURCE
#define _GNU_SOURCE
#include <stdarg.h>
#include <stdio.h>

typedef FILE *t1;

FILE *f1(const char *pathname, const char *mode) {
  return fopen(pathname, mode);
}

int f2(FILE *stream) { return fclose(stream); }

size_t f5(void *ptr, size_t size, size_t nmemb, FILE *stream) {
  return fread(ptr, size, nmemb, stream);
}

size_t f6(const void *ptr, size_t size, size_t nmemb, FILE *stream) {
  return fwrite(ptr, size, nmemb, stream);
}

int f7(FILE *stream) { return fflush(stream); }

FILE *f8(void) { return stdout; }

FILE *f9(void) { return stderr; }

FILE *f10(void) { return stdin; }

int f11(int c, FILE *stream) { return fputc(c, stream); }

int f12(const char *s, FILE *stream) { return fputs(s, stream); }

int f13(const char *s) { return puts(s); }

int (*f28)(FILE *, const char *, ...) = fprintf;

int (*f29)(const char *, ...) = printf;
typedef fpos_t t3;

int f100(const char *a0) { return remove(a0); }

void f101(const char *s) { return perror(s); }

int f104(FILE *stream) { return fgetc(stream); }

int f106(int c, FILE *stream) { return ungetc(c, stream); }
