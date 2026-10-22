// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <sys/sem.h>

sembuf f1(unsigned short a0, short a1, short a2) { return {a0, a1, a2}; }
