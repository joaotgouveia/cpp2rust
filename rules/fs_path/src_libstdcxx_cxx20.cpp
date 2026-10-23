#include <algorithm>
#include <filesystem>
#include <ios>
#include <iosfwd>
#include <string_view>
#include <system_error>
#include <utility>

std::u8string f91(const std::filesystem::path &a0) { return a0.u8string(); }

std::u8string f92(const std::filesystem::path &a0) {
  return a0.generic_u8string();
}

std::strong_ordering f93(const std::filesystem::path &a0,
                         const std::filesystem::path &a1) {
  return operator<=>(a0, a1);
}
