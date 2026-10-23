#include <algorithm>
#include <utility>

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f25(std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::input_range<std::ranges::subrange<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f26(const std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::input_range<const std::ranges::subrange<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f27(std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::input_range<std::ranges::subrange<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>)
{
  return a0.cend();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f28(const std::ranges::subrange<T1, T2, T3> &a0)
  requires(std::ranges::input_range<const std::ranges::subrange<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::subrange<T1, T2, T3>> &&
           std::same_as<std::ranges::subrange<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::subrange<T1, T2, T3>>>)
{
  return a0.cend();
}
