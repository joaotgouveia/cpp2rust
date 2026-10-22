#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <filesystem>
#include <string.h>
#include <string>
#include <string_view>
#include <utility>

std::string f71(const std::filesystem::path &a0) { return a0.u8string(); }

std::string f72(const std::filesystem::path &a0) {
  return a0.generic_u8string();
}

bool f73(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return a0 >= a1;
}

bool f74(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return a0 > a1;
}

bool f75(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return a0 <= a1;
}

bool f76(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return a0 < a1;
}

bool f77(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return a0 != a1;
}
