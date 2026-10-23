#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, std::size_t T2>
auto f27(std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::elements_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>>)
{
  return a0.cbegin();
}

template <typename T1, std::size_t T2>
auto f28(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::input_range<
              const std::ranges::elements_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>>)
{
  return a0.cbegin();
}

template <typename T1, std::size_t T2>
auto f29(std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::elements_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>>)
{
  return a0.cend();
}

template <typename T1, std::size_t T2>
auto f30(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::input_range<
              const std::ranges::elements_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>>)
{
  return a0.cend();
}
