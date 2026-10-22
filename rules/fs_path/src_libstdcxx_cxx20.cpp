#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <filesystem>
#include <ios>
#include <iosfwd>
#include <string_view>
#include <system_error>
#include <utility>

std::u8string f101(const std::filesystem::path &a0) { return a0.u8string(); }

std::u8string f102(const std::filesystem::path &a0) {
  return a0.generic_u8string();
}

std::strong_ordering f103(const std::filesystem::path &a0,
                          const std::filesystem::path &a1) {
  return a0 <=> a1;
}
