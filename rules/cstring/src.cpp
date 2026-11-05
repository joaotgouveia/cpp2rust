// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstring>

const char *f2001(const char *a0, int a1) { return ::strchr(a0, a1); }

char *f2002(char *a0, int a1) { return ::strchr(a0, a1); }

const char *f2003(const char *a0, int a1) { return ::strrchr(a0, a1); }

char *f2004(char *a0, int a1) { return ::strrchr(a0, a1); }

const char *f2005(const char *a0, const char *a1) { return ::strpbrk(a0, a1); }

char *f2006(char *a0, const char *a1) { return ::strpbrk(a0, a1); }

const char *f2007(const char *a0, const char *a1) { return ::strstr(a0, a1); }

char *f2008(char *a0, const char *a1) { return ::strstr(a0, a1); }

const void *f2009(const void *a0, int a1, std::size_t a2) {
  return ::memchr(a0, a1, a2);
}

void *f2010(void *a0, int a1, std::size_t a2) { return ::memchr(a0, a1, a2); }
