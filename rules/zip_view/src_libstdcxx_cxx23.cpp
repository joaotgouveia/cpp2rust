#include <algorithm>
#include <ranges>
#include <utility>

template <typename... T1>
auto f9(std::ranges::zip_view<T1...> &a0)
  requires((!(std::ranges::__detail::__simple_view<T1> && ...))) &&
          ((std::ranges::view<T1> && ...) && (sizeof...(T1) > 0))
{
  return a0.begin();
}

template <typename... T1>
auto f10(std::ranges::zip_view<T1...> &a0)
  requires((!(std::ranges::__detail::__simple_view<T1> && ...))) &&
          ((std::ranges::view<T1> && ...) && (sizeof...(T1) > 0))
{
  return a0.end();
}

template <typename... T1>
bool f11(std::ranges::zip_view<T1...> &a0)
  requires(std::ranges::forward_range<std::ranges::zip_view<T1...>> &&
           (!std::ranges::sized_range<std::ranges::zip_view<T1...>>)) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.empty();
}

template <typename... T1>
bool f12(std::ranges::zip_view<T1...> &a0)
  requires(std::ranges::sized_range<std::ranges::zip_view<T1...>>) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.empty();
}

template <typename... T1>
bool f13(const std::ranges::zip_view<T1...> &a0)
  requires(std::ranges::forward_range<const std::ranges::zip_view<T1...>> &&
           (!std::ranges::sized_range<const std::ranges::zip_view<T1...>>)) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.empty();
}

template <typename... T1>
bool f14(const std::ranges::zip_view<T1...> &a0)
  requires(std::ranges::sized_range<const std::ranges::zip_view<T1...>>) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.empty();
}

template <typename... T1> bool f15(std::ranges::zip_view<T1...> &a0) {
  return static_cast<bool>(a0);
}

template <typename... T1> bool f16(const std::ranges::zip_view<T1...> &a0) {
  return static_cast<bool>(a0);
}

template <typename... T1>
auto f17(std::ranges::zip_view<T1...> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::zip_view<T1...>>>) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.data();
}

template <typename... T1>
auto f18(const std::ranges::zip_view<T1...> &a0)
  requires(std::ranges::range<const std::ranges::zip_view<T1...>> &&
           std::contiguous_iterator<
               std::ranges::iterator_t<const std::ranges::zip_view<T1...>>>) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.data();
}

template <typename... T1>
decltype(auto) f19(std::ranges::zip_view<T1...> &a0)
  requires(std::ranges::forward_range<std::ranges::zip_view<T1...>>) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.front();
}

template <typename... T1>
decltype(auto) f20(const std::ranges::zip_view<T1...> &a0)
  requires(std::ranges::forward_range<const std::ranges::zip_view<T1...>>) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.front();
}

template <typename... T1>
decltype(auto) f21(std::ranges::zip_view<T1...> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::zip_view<T1...>> &&
           std::ranges::common_range<std::ranges::zip_view<T1...>>) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.back();
}

template <typename... T1>
decltype(auto) f22(const std::ranges::zip_view<T1...> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::zip_view<T1...>> &&
           std::ranges::common_range<const std::ranges::zip_view<T1...>>) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.back();
}

template <typename... T1>
auto f23(std::ranges::zip_view<T1...> &a0)
  requires(std::ranges::input_range<std::ranges::zip_view<T1...>>) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.cbegin();
}

template <typename... T1>
auto f24(const std::ranges::zip_view<T1...> &a0)
  requires(std::ranges::input_range<const std::ranges::zip_view<T1...>>) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.cbegin();
}

template <typename... T1>
auto f25(std::ranges::zip_view<T1...> &a0)
  requires(std::ranges::input_range<std::ranges::zip_view<T1...>>) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.cend();
}

template <typename... T1>
auto f26(const std::ranges::zip_view<T1...> &a0)
  requires(std::ranges::input_range<const std::ranges::zip_view<T1...>>) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.cend();
}
