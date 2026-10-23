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

template <typename T1,
          std::__enable_if_t<std::is_error_code_enum<T1>::value, int> T2 = 0,
          typename std::enable_if<
              (!(std::is_same<T1, const std::error_code &>::value) &&
               !(std::is_same<T1, std::error_code &&>::value)),
              int>::type T3 = 0>
std::error_code f5(T1 a0) {
  return std::error_code(a0);
}

template <typename T1,
          std::__enable_if_t<std::is_error_code_enum<T1>::value, int> T2 = 0,
          typename std::enable_if<
              (!(std::is_same<T1, const std::error_code &>::value) &&
               !(std::is_same<T1, std::error_code &&>::value)),
              int>::type T3 = 0>
std::error_code &f6(std::error_code &a0, T1 a1) {
  return a0.template operator= <T1>(a1);
}

bool f7(const std::error_code &a0, const std::error_code &a1) {
  return std::operator<(a0, a1);
}
