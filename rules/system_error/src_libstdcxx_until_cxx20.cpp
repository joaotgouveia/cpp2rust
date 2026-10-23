#include <string>
#include <system_error>
#include <utility>

bool f48(const std::error_category &a0, const std::error_category &a1) {
  return a0.operator<(a1);
}

bool f49(const std::error_category &a0, const std::error_category &a1) {
  return a0.operator!=(a1);
}

bool f50(const std::error_code &a0, const std::error_condition &a1) {
  return a0 != a1;
}

bool f51(const std::error_code &a0, const std::error_code &a1) {
  return a0 != a1;
}

bool f52(const std::error_condition &a0, const std::error_code &a1) {
  return a0 != a1;
}

bool f53(const std::error_condition &a0, const std::error_condition &a1) {
  return a0 != a1;
}

bool f54(const std::error_code &a0, const std::error_code &a1) {
  return a0 < a1;
}

bool f55(const std::error_condition &a0, const std::error_condition &a1) {
  return a0 < a1;
}

bool f56(const std::error_condition &a0, const std::error_code &a1) {
  return a0 == a1;
}
