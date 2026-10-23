#include <compare>
#include <string.h>
#include <system_error>
#include <utility>

template <typename T1,
          std::__enable_if_t<std::is_error_code_enum<T1>::value, int> T2 = 0>
std::error_code f24(T1 a0)
  requires(!(std::is_same<T1, const std::error_code &>::value) &&
           !(std::is_same<T1, std::error_code &&>::value))
{
  return std::error_code(a0);
}

template <typename T1,
          std::__enable_if_t<std::is_error_code_enum<T1>::value, int> T2 = 0>
std::error_code &f25(std::error_code &a0, T1 a1)
  requires(!(std::is_same<T1, const std::error_code &>::value) &&
           !(std::is_same<T1, std::error_code &&>::value))
{
  return a0.template operator= <T1>(a1);
}

std::strong_ordering f26(const std::error_code &a0, const std::error_code &a1) {
  return std::operator<=>(a0, a1);
}
