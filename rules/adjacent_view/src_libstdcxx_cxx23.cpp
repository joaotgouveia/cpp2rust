#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, std::size_t T2>
auto f11(std::ranges::adjacent_view<T1, T2> &a0)
  requires((!std::ranges::__detail::__simple_view<T1>)) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.begin();
}

template <typename T1, std::size_t T2>
auto f12(std::ranges::adjacent_view<T1, T2> &a0)
  requires((!std::ranges::__detail::__simple_view<T1>)) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.end();
}

template <typename T1, std::size_t T2>
bool f13(std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::adjacent_view<T1, T2>> &&
           (!std::ranges::sized_range<std::ranges::adjacent_view<T1, T2>>)) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.empty();
}

template <typename T1, std::size_t T2>
bool f14(std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::adjacent_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.empty();
}

template <typename T1, std::size_t T2>
bool f15(const std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
               const std::ranges::adjacent_view<T1, T2>> &&
           (!std::ranges::sized_range<
               const std::ranges::adjacent_view<T1, T2>>)) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.empty();
}

template <typename T1, std::size_t T2>
bool f16(const std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::sized_range<
              const std::ranges::adjacent_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.empty();
}

template <typename T1, std::size_t T2>
bool f17(std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && (T2 > 0))
{
  return static_cast<bool>(a0);
}

template <typename T1, std::size_t T2>
bool f18(const std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && (T2 > 0))
{
  return static_cast<bool>(a0);
}

template <typename T1, std::size_t T2>
auto f19(std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.data();
}

template <typename T1, std::size_t T2>
auto f20(const std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::range<const std::ranges::adjacent_view<T1, T2>> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               const std::ranges::adjacent_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.data();
}

template <typename T1, std::size_t T2>
decltype(auto) f21(std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::adjacent_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.front();
}

template <typename T1, std::size_t T2>
decltype(auto) f22(const std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              const std::ranges::adjacent_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.front();
}

template <typename T1, std::size_t T2>
decltype(auto) f23(std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::adjacent_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::adjacent_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.back();
}

template <typename T1, std::size_t T2>
decltype(auto) f24(const std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::adjacent_view<T1, T2>> &&
           std::ranges::common_range<
               const std::ranges::adjacent_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.back();
}

template <typename T1, std::size_t T2>
auto f25(std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::adjacent_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.cbegin();
}

template <typename T1, std::size_t T2>
auto f26(const std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::input_range<
              const std::ranges::adjacent_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.cbegin();
}

template <typename T1, std::size_t T2>
auto f27(std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::adjacent_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.cend();
}

template <typename T1, std::size_t T2>
auto f28(const std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::input_range<
              const std::ranges::adjacent_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.cend();
}
