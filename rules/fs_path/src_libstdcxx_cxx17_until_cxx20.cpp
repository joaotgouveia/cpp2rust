#include <algorithm>
#include <filesystem>
#include <string>
#include <system_error>
#include <utility>

std::string f78(const std::filesystem::path &a0) { return a0.u8string(); }

std::string f79(const std::filesystem::path &a0) {
  return a0.generic_u8string();
}

bool f80(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return a0 >= a1;
}

bool f81(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return a0 > a1;
}

bool f82(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return a0 <= a1;
}

bool f83(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return a0 < a1;
}

bool f84(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return a0 != a1;
}
