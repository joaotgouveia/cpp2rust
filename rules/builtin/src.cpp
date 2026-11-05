// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstdio>

template <typename... Args>
int f100(FILE *a0, const char *a1, Args... va) {
  return __builtin_fprintf(a0, a1, va...);
}
