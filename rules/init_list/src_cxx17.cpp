// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <charconv>
#include <memory_resource>
#include <system_error>

std::from_chars_result f12(const char *a0, std::errc a1) { return {a0, a1}; }

std::to_chars_result f13(char *a0, std::errc a1) { return {a0, a1}; }

std::pmr::pool_options f14(std::size_t a0, std::size_t a1) { return {a0, a1}; }
