#include <string>
#include <system_error>
#include <utility>

bool f46(const std::error_category &a0, const std::error_category &a1) {
  return a0.operator<(a1);
}

bool f47(const std::error_category &a0, const std::error_category &a1) {
  return a0.operator!=(a1);
}

template <typename T1,
          typename T2 = std::__enable_if_t<std::is_error_code_enum<T1>::value>,
          typename std::enable_if<
              (!(std::is_same<T1, const std::error_code &>::value)), int>::type
              T3 = 0>
std::error_code f48(T1 a0) {
  return std::error_code(std::move(a0));
}

template <
    typename T1,
    typename T2 = std::__enable_if_t<std::is_error_condition_enum<T1>::value>,
    typename std::enable_if<
        (!(std::is_same<T1, const std::error_condition &>::value)), int>::type
        T3 = 0>
std::error_condition f49(T1 a0) {
  return std::error_condition(std::move(a0));
}

bool f50(const std::error_code &a0, const std::error_condition &a1) {
  return std::operator!=(a0, a1);
}

bool f51(const std::error_code &a0, const std::error_code &a1) {
  return std::operator!=(a0, a1);
}

bool f52(const std::error_condition &a0, const std::error_code &a1) {
  return std::operator!=(a0, a1);
}

bool f53(const std::error_condition &a0, const std::error_condition &a1) {
  return std::operator!=(a0, a1);
}

bool f54(const std::error_code &a0, const std::error_code &a1) {
  return std::operator<(a0, a1);
}

bool f55(const std::error_condition &a0, const std::error_condition &a1) {
  return std::operator<(a0, a1);
}

bool f56(const std::error_condition &a0, const std::error_code &a1) {
  return std::operator==(a0, a1);
}
