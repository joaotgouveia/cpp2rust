// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#undef _GNU_SOURCE
#define _GNU_SOURCE
#include <unistd.h>

int f1(int fd) { return close(fd); }

int f4(const char *pathname) { return unlink(pathname); }

size_t f12(int name, char *buf, size_t len) { return confstr(name, buf, len); }

pid_t f17(void) { return getpid(); }

#ifdef _CS_GNU_LIBC_VERSION
int f18(void) { return _CS_GNU_LIBC_VERSION; }
#endif
