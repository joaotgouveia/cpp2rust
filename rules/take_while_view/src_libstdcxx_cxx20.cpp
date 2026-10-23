#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::take_while_view<T1, T2> f9()
  requires((std::default_initializable<T1> &&
            std::default_initializable<T2>)) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return std::ranges::take_while_view<T1, T2>();
}

template <typename T1, typename T2>
auto f10(std::ranges::take_while_view<T1, T2> &a0)
  requires((!std::ranges::__detail::__simple_view<T1>)) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f11(std::ranges::take_while_view<T1, T2> &a0)
  requires((!std::ranges::__detail::__simple_view<T1>)) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.end();
}

template <typename T1, typename T2>
bool f12(std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::take_while_view<T1, T2>> &&
           (!std::ranges::sized_range<std::ranges::take_while_view<T1, T2>>)) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f13(std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::take_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f14(const std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
               const std::ranges::take_while_view<T1, T2>> &&
           (!std::ranges::sized_range<
               const std::ranges::take_while_view<T1, T2>>)) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f15(const std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::sized_range<
              const std::ranges::take_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f16(std::ranges::take_while_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f17(const std::ranges::take_while_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
auto f18(std::ranges::take_while_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::take_while_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.data();
}

template <typename T1, typename T2>
auto f19(const std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::range<const std::ranges::take_while_view<T1, T2>> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               const std::ranges::take_while_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.data();
}

template <typename T1, typename T2>
auto f20(std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::take_while_view<T1, T2>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<std::ranges::take_while_view<T1, T2>>,
               std::ranges::iterator_t<
                   std::ranges::take_while_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.size();
}

template <typename T1, typename T2>
auto f21(const std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
               const std::ranges::take_while_view<T1, T2>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<
                   const std::ranges::take_while_view<T1, T2>>,
               std::ranges::iterator_t<
                   const std::ranges::take_while_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.size();
}

template <typename T1, typename T2>
decltype(auto) f22(std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::take_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f23(const std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              const std::ranges::take_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f24(std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::take_while_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::take_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.back();
}

template <typename T1, typename T2>
decltype(auto) f25(const std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::take_while_view<T1, T2>> &&
           std::ranges::common_range<
               const std::ranges::take_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.back();
}
