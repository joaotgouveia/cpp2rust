#include <compare>
#include <string.h>
#include <system_error>
#include <utility>

template <
    typename T1,
    std::__enable_if_t<std::is_error_condition_enum<T1>::value, int> T2 = 0>
std::error_condition f19(T1 a0)
  requires(!(std::is_same<T1, const std::error_condition &>::value) &&
           !(std::is_same<T1, std::error_condition &&>::value))
{
  return std::error_condition(std::move(a0));
}

template <
    typename T1,
    std::__enable_if_t<std::is_error_condition_enum<T1>::value, int> T2 = 0>
std::error_condition &f20(std::error_condition &a0, T1 a1)
  requires(!(std::is_same<T1, const std::error_condition &>::value) &&
           !(std::is_same<T1, std::error_condition &&>::value))
{
  return a0.template operator= <T1>(std::move(a1));
}

std::strong_ordering f21(const std::error_condition &a0,
                         const std::error_condition &a1) {
  return std::operator<=>(a0, a1);
}
