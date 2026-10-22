#include "../common/init.h"
#include "../common/traits.h"
#include <filesystem>
#include <string.h>
#include <system_error>
#include <utility>

using t1 = std::filesystem::filesystem_error;

std::filesystem::filesystem_error f1(const std::string &a0,
                                     std::error_code a1) {
  return std::filesystem::filesystem_error(a0, std::move(a1));
}

std::filesystem::filesystem_error
f2(const std::string &a0, const std::filesystem::path &a1, std::error_code a2) {
  return std::filesystem::filesystem_error(a0, a1, std::move(a2));
}

std::filesystem::filesystem_error f3(const std::string &a0,
                                     const std::filesystem::path &a1,
                                     const std::filesystem::path &a2,
                                     std::error_code a3) {
  return std::filesystem::filesystem_error(a0, a1, a2, std::move(a3));
}

const std::filesystem::path &f4(const std::filesystem::filesystem_error &a0) {
  return a0.path1();
}

const std::filesystem::path &f5(const std::filesystem::filesystem_error &a0) {
  return a0.path2();
}

std::filesystem::filesystem_error
f6(const std::filesystem::filesystem_error &a0) {
  return std::filesystem::filesystem_error(a0);
}

const char *f7(const std::filesystem::filesystem_error &a0) {
  return a0.what();
}

std::filesystem::filesystem_error &
f8(std::filesystem::filesystem_error &a0,
   const std::filesystem::filesystem_error &a1) {
  return a0.operator=(a1);
}

const std::error_code &f9(const std::filesystem::filesystem_error &a0) {
  return a0.code();
}
