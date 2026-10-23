#include <compare>
#include <cstddef>
#include <filesystem>
#include <string.h>
#include <string>
#include <string_view>
#include <utility>

std::u8string f80(const std::filesystem::path &a0) { return a0.u8string(); }

std::u8string f81(const std::filesystem::path &a0) {
  return a0.generic_u8string();
}

std::strong_ordering f82(const std::filesystem::path &a0,
                         const std::filesystem::path &a1) {
  return operator<=>(a0, a1);
}
