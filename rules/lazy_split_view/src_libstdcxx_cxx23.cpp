#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
auto f26(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::lazy_split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f27(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::input_range<
              const std::ranges::lazy_split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f28(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::lazy_split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.cend();
}

template <typename T1, typename T2>
auto f29(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::input_range<
              const std::ranges::lazy_split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.cend();
}
