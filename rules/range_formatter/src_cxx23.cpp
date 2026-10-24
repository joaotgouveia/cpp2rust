#include <format>
#include <string_view>
#include <utility>

template <typename T1, typename T2> using t1 = std::range_formatter<T1, T2>;

template <typename T1, typename T2>
void f1(std::range_formatter<T1, T2> &a0, std::basic_string_view<T2> a1)
  requires(std::same_as<std::remove_cvref_t<T1>, T1> &&
           std::formattable<T1, T2>)
{
  return a0.set_separator(std::move(a1));
}

template <typename T1, typename T2>
void f2(std::range_formatter<T1, T2> &a0, std::basic_string_view<T2> a1,
        std::basic_string_view<T2> a2)
  requires(std::same_as<std::remove_cvref_t<T1>, T1> &&
           std::formattable<T1, T2>)
{
  return a0.set_brackets(std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
std::formatter<T1, T2> &f3(std::range_formatter<T1, T2> &a0)
  requires(std::same_as<std::remove_cvref_t<T1>, T1> &&
           std::formattable<T1, T2>)
{
  return a0.underlying();
}

template <typename T1, typename T2>
const std::formatter<T1, T2> &f4(const std::range_formatter<T1, T2> &a0)
  requires(std::same_as<std::remove_cvref_t<T1>, T1> &&
           std::formattable<T1, T2>)
{
  return a0.underlying();
}
