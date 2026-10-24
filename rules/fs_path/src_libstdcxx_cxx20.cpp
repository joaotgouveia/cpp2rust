#include <algorithm>
#include <barrier>
#include <filesystem>
#include <ios>
#include <iosfwd>
#include <istream>
#include <string_view>
#include <system_error>
#include <utility>

std::u8string f93(const std::filesystem::path &a0) { return a0.u8string(); }

std::u8string f94(const std::filesystem::path &a0) {
  return a0.generic_u8string();
}

std::strong_ordering f95(const std::filesystem::path &a0,
                         const std::filesystem::path &a1) {
  return operator<=>(a0, a1);
}
