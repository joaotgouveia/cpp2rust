#include <format>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
typename T3::iterator f7(std::range_formatter<T1, T2> &a0, T3 &a1)
  requires(std::same_as<std::remove_cvref_t<T1>, T1> &&
           std::formattable<T1, T2>)
{
  return a0.template parse<T3>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator f8(const std::range_formatter<T1, T2> &a0, T3 &&a1,
                         T4 &a2)
  requires(std::formattable<std::ranges::range_reference_t<T3>, T2> &&
           std::same_as<std::remove_cvref_t<std::ranges::range_reference_t<T3>>,
                        T1>) &&
          (std::same_as<std::remove_cvref_t<T1>, T1> &&
           std::formattable<T1, T2>)
{
  return a0.template format<T3, T4>(std::forward<T3>(a1), a2);
}
