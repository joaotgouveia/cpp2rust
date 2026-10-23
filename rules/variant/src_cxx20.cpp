#include <algorithm>
#include <compare>
#include <cstddef>
#include <initializer_list>
#include <tuple>
#include <type_traits>
#include <utility>
#include <variant>

template <typename... T1>
std::common_comparison_category_t<std::compare_three_way_result_t<T1>...>
f26(const std::variant<T1...> &a0, const std::variant<T1...> &a1)
  requires((std::three_way_comparable<T1> && ...))
{
  return std::operator<=> <T1...>(a0, a1);
}
