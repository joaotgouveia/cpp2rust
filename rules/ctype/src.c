// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <ctype.h>

int f1(int a0) { return toupper(a0); }

int f2(int a0) { return tolower(a0); }

int f3(int a0) { return (isalnum)(a0); }
