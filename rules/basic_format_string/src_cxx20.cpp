#include <format>
#include <string_view>
#include <utility>

template <typename T1, typename... T2>
using t1 = std::basic_format_string<T1, T2...>;

template <typename T1, typename... T2>
std::basic_string_view<T1> f1(const std::basic_format_string<T1, T2...> &a0) {
  return a0.get();
}

template <typename T1, typename... T2, typename T3>
consteval std::basic_format_string<T1, T2...> f2(const T3 &a0)
  requires(std::convertible_to<const T3 &, std::basic_string_view<T1>>)
{
  return std::basic_format_string<T1, T2...>(a0);
}
