#include <compare>
#include <iosfwd>
#include <system_error>
#include <utility>

std::strong_ordering f57(const std::error_category &a0,
                         const std::error_category &a1) {
  return a0.operator<=>(a1);
}

template <typename T1,
          typename T2 = std::__enable_if_t<std::is_error_code_enum<T1>::value>>
std::error_code f58(T1 a0)
  requires(!(std::is_same<T1, const std::error_code &>::value))
{
  return std::error_code(a0);
}

template <typename T1, typename T2 = std::__enable_if_t<
                           std::is_error_condition_enum<T1>::value>>
std::error_condition f59(T1 a0)
  requires(!(std::is_same<T1, const std::error_condition &>::value))
{
  return std::error_condition(a0);
}

std::strong_ordering f60(const std::error_code &a0, const std::error_code &a1) {
  return std::operator<=>(a0, a1);
}

std::strong_ordering f61(const std::error_condition &a0,
                         const std::error_condition &a1) {
  return std::operator<=>(a0, a1);
}
