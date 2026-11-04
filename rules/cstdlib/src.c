// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#undef _GNU_SOURCE
#define _GNU_SOURCE
#include <stdlib.h>

void f1001(void) { return abort(); }

void f1002(void *a0) { return free(a0); }

void *f1003(size_t a0) { return malloc(a0); }

void *f1005(size_t nmemb, size_t size) { return calloc(nmemb, size); }

char *f1006(const char *name) { return getenv(name); }

int f1007(const char *name, const char *value, int overwrite) {
  return setenv(name, value, overwrite);
}

void *f1008(const void *key, const void *base, size_t nmemb, size_t size,
         int (*compar)(const void *, const void *)) {
  return bsearch(key, base, nmemb, size, compar);
}

int f1011(char *a0) { return mkstemp(a0); }

void f1014(int a0) { return quick_exit(a0); }

int f1015(void (*a0)(void)) { return at_quick_exit(a0); }

int f1016(int a0) { return abs(a0); }

int f1021(void) { return rand(); }
