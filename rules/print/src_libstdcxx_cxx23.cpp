#include <format>
#include <print>
#include <string_view>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

void f5() { return std::println(); }

void f6(std::FILE *a0) { return std::println(a0); }

void f7(std::string_view a0, std::format_args a1) {
  return std::vprint_nonunicode(std::move(a0), std::move(a1));
}

void f8(std::FILE *a0, std::string_view a1, std::format_args a2) {
  return std::vprint_nonunicode(a0, std::move(a1), std::move(a2));
}

void f9(std::string_view a0, std::format_args a1) {
  return std::vprint_unicode(std::move(a0), std::move(a1));
}

void f10(std::FILE *a0, std::string_view a1, std::format_args a2) {
  return std::vprint_unicode(a0, std::move(a1), std::move(a2));
}
