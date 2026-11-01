// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <ios>

using t1 = std::ios_base::openmode;

std::ios_base::openmode f1() { return std::ios_base::app; }

std::ios_base::openmode f2() { return std::ios_base::ate; }

std::ios_base::openmode f3() { return std::ios_base::binary; }

std::ios_base::openmode f4() { return std::ios_base::in; }

std::ios_base::openmode f5() { return std::ios_base::out; }

std::ios_base::openmode f6() { return std::ios_base::trunc; }

std::ios_base::openmode f7() { return std::ios_base::noreplace; }
