#include "../common/init.h"
#include "../common/traits.h"
#include <compare>
#include <cstddef>
#include <filesystem>
#include <string.h>
#include <string>
#include <string_view>
#include <utility>

std::u8string f86(const std::filesystem::path &a0) { return a0.u8string(); }

std::u8string f87(const std::filesystem::path &a0) {
  return a0.generic_u8string();
}

std::strong_ordering f88(const std::filesystem::path &a0,
                         const std::filesystem::path &a1) {
  return a0 <=> a1;
}
