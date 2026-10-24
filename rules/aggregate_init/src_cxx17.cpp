#include <charconv>
#include <cstddef>
#include <memory_resource>
#include <system_error>

std::from_chars_result f3(const char *a0, std::errc a1) { return {a0, a1}; }

std::to_chars_result f4(char *a0, std::errc a1) { return {a0, a1}; }

std::pmr::pool_options f5(std::size_t a0, std::size_t a1) { return {a0, a1}; }
