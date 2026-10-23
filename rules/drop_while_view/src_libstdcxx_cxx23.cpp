#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
auto f24(std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::drop_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f25(const std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::input_range<
              const std::ranges::drop_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f26(std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::drop_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.cend();
}

template <typename T1, typename T2>
auto f27(const std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::input_range<
              const std::ranges::drop_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.cend();
}
