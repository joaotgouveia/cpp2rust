#include <algorithm>
#include <filesystem>
#include <ios>
#include <string>
#include <system_error>
#include <utility>

std::string f84(const std::filesystem::path &a0) { return a0.u8string(); }

std::string f85(const std::filesystem::path &a0) {
  return a0.generic_u8string();
}

bool f86(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator>=(a0, a1);
}

bool f87(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator>(a0, a1);
}

bool f88(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator<=(a0, a1);
}

bool f89(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator<(a0, a1);
}

bool f90(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator!=(a0, a1);
}
