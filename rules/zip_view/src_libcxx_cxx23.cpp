#include <ranges>
#include <utility>

template <typename... T1>
auto f27(std::ranges::zip_view<T1...> &a0)
  requires((!(std::ranges::__simple_view<T1> && ...))) &&
          ((std::ranges::view<T1> && ...) && (sizeof...(T1) > 0))
{
  return a0.begin();
}

template <typename... T1>
auto f28(std::ranges::zip_view<T1...> &a0)
  requires((!(std::ranges::__simple_view<T1> && ...))) &&
          ((std::ranges::view<T1> && ...) && (sizeof...(T1) > 0))
{
  return a0.end();
}

template <typename... T1, typename T2 = std::ranges::zip_view<T1...>>
bool f29(std::ranges::zip_view<T1...> &a0)
  requires(std::ranges::sized_range<std::ranges::zip_view<T1...>> ||
           std::ranges::forward_range<std::ranges::zip_view<T1...>>) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.empty();
}

template <typename... T1, typename T2 = std::ranges::zip_view<T1...>>
bool f30(const std::ranges::zip_view<T1...> &a0)
  requires(std::ranges::sized_range<std::ranges::zip_view<T1...> const> ||
           std::ranges::forward_range<std::ranges::zip_view<T1...> const>) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.empty();
}

template <typename... T1, typename T2 = std::ranges::zip_view<T1...>>
bool f31(std::ranges::zip_view<T1...> &a0) {
  return static_cast<bool>(a0);
}

template <typename... T1, typename T2 = std::ranges::zip_view<T1...>>
bool f32(const std::ranges::zip_view<T1...> &a0) {
  return static_cast<bool>(a0);
}

template <typename... T1, typename T2 = std::ranges::zip_view<T1...>>
auto f33(std::ranges::zip_view<T1...> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::zip_view<T1...>>>) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.data();
}

template <typename... T1, typename T2 = std::ranges::zip_view<T1...>>
auto f34(const std::ranges::zip_view<T1...> &a0)
  requires(std::ranges::range<std::ranges::zip_view<T1...> const> &&
           std::contiguous_iterator<
               std::ranges::iterator_t<std::ranges::zip_view<T1...> const>>) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.data();
}

template <typename... T1, typename T2 = std::ranges::zip_view<T1...>>
decltype(auto) f35(std::ranges::zip_view<T1...> &a0)
  requires(std::ranges::forward_range<std::ranges::zip_view<T1...>>) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.front();
}

template <typename... T1, typename T2 = std::ranges::zip_view<T1...>>
decltype(auto) f36(const std::ranges::zip_view<T1...> &a0)
  requires(std::ranges::forward_range<std::ranges::zip_view<T1...> const>) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.front();
}

template <typename... T1, typename T2 = std::ranges::zip_view<T1...>>
decltype(auto) f37(std::ranges::zip_view<T1...> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::zip_view<T1...>> &&
           std::ranges::common_range<std::ranges::zip_view<T1...>>) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.back();
}

template <typename... T1, typename T2 = std::ranges::zip_view<T1...>>
decltype(auto) f38(const std::ranges::zip_view<T1...> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::zip_view<T1...> const> &&
           std::ranges::common_range<std::ranges::zip_view<T1...> const>) &&
          (std::is_class_v<std::ranges::zip_view<T1...>> &&
           std::same_as<std::ranges::zip_view<T1...>,
                        std::remove_cv_t<std::ranges::zip_view<T1...>>>)
{
  return a0.back();
}
