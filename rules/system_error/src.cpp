#include "../common/init.h"
#include "../common/traits.h"
#include <string>
#include <system_error>
#include <utility>

using t1 = std::system_error;

std::system_error f1(std::error_code a0) {
  return std::system_error(std::move(a0));
}

std::system_error f2(std::error_code a0, const std::string &a1) {
  return std::system_error(std::move(a0), a1);
}

std::system_error f3(std::error_code a0, const char *a1) {
  return std::system_error(std::move(a0), a1);
}

std::system_error f4(int a0, const std::error_category &a1, const char *a2) {
  return std::system_error(a0, a1, a2);
}

std::system_error f5(int a0, const std::error_category &a1) {
  return std::system_error(a0, a1);
}

std::system_error f6(int a0, const std::error_category &a1,
                     const std::string &a2) {
  return std::system_error(a0, a1, a2);
}

std::system_error f7(const std::system_error &a0) {
  return std::system_error(a0);
}

std::system_error &f8(std::system_error &a0, const std::system_error &a1) {
  return a0.operator=(a1);
}

const std::error_code &f9(const std::system_error &a0) { return a0.code(); }

const char *f10(const std::system_error &a0) { return a0.what(); }
