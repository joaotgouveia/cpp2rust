#include <string.h>
#include <system_error>
#include <utility>

bool f1(const std::error_code &a0, const std::error_condition &a1) {
  return std::operator!=(a0, a1);
}

bool f2(const std::error_code &a0, const std::error_code &a1) {
  return std::operator!=(a0, a1);
}

bool f3(const std::error_condition &a0, const std::error_code &a1) {
  return std::operator!=(a0, a1);
}

bool f4(const std::error_condition &a0, const std::error_code &a1) {
  return std::operator==(a0, a1);
}

bool f5(const std::error_code &a0, const std::error_code &a1) {
  return std::operator<(a0, a1);
}
