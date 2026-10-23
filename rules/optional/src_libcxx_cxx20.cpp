#include <compare>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f128(const std::optional<T1> &a0,
                                             const T2 &a1)
  requires((!std::__is_derived_from_optional<T2>) &&
           std::three_way_comparable_with<T1, T2>)
{
  return std::operator<=> <T1, T2>(a0, a1);
}
