#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <filesystem>
#include <iosfwd>
#include <string>
#include <string_view>
#include <system_error>
#include <utility>

std::u8string f85(const std::filesystem::path &a0) { return a0.u8string(); }

std::u8string f86(const std::filesystem::path &a0) {
  return a0.generic_u8string();
}

std::strong_ordering f87(const std::filesystem::path &a0,
                         const std::filesystem::path &a1) {
  return a0 <=> a1;
}
