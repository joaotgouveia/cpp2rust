#include <algorithm>
#include <istream>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3>
auto f20(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::input_range<
              std::ranges::basic_istream_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<
               std::ranges::basic_istream_view<T1, T2, T3>,
               std::remove_cv_t<std::ranges::basic_istream_view<T1, T2, T3>>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
auto f21(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::input_range<
              const std::ranges::basic_istream_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<
               std::ranges::basic_istream_view<T1, T2, T3>,
               std::remove_cv_t<std::ranges::basic_istream_view<T1, T2, T3>>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
auto f22(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::input_range<
              std::ranges::basic_istream_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<
               std::ranges::basic_istream_view<T1, T2, T3>,
               std::remove_cv_t<std::ranges::basic_istream_view<T1, T2, T3>>>)
{
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
auto f23(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::input_range<
              const std::ranges::basic_istream_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<
               std::ranges::basic_istream_view<T1, T2, T3>,
               std::remove_cv_t<std::ranges::basic_istream_view<T1, T2, T3>>>)
{
  return a0.cend();
}
