#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <filesystem>
#include <ios>
#include <string>
#include <system_error>
#include <utility>

std::string f92(const std::filesystem::path &a0) { return a0.u8string(); }

std::string f93(const std::filesystem::path &a0) {
  return a0.generic_u8string();
}

bool f94(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return a0 >= a1;
}

bool f95(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return a0 > a1;
}

bool f96(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return a0 <= a1;
}

bool f97(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return a0 < a1;
}

bool f98(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return a0 != a1;
}

template <typename T1> std::filesystem::path f99(const T1 &a0) {
  return std::filesystem::u8path<T1>(a0);
}

template <typename T1> std::filesystem::path f100(T1 a0, T1 a1) {
  return std::filesystem::u8path<T1>(a0, a1);
}
