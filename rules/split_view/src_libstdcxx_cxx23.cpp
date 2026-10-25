#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
auto f33(std::ranges::split_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::split_view<T1, T2>> &&
           std::same_as<std::ranges::split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f34(const std::ranges::split_view<T1, T2> &a0)
  requires(std::ranges::input_range<const std::ranges::split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::split_view<T1, T2>> &&
           std::same_as<std::ranges::split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f35(std::ranges::split_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::split_view<T1, T2>> &&
           std::same_as<std::ranges::split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return a0.cend();
}

template <typename T1, typename T2>
auto f36(const std::ranges::split_view<T1, T2> &a0)
  requires(std::ranges::input_range<const std::ranges::split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::split_view<T1, T2>> &&
           std::same_as<std::ranges::split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return a0.cend();
}
