#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename... T2>
using t1 = std::ranges::cartesian_product_view<T1, T2...>;

template <typename T1, typename... T2>
std::ranges::cartesian_product_view<T1, T2...> f1()
  requires((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return std::ranges::cartesian_product_view<T1, T2...>();
}

template <typename T1, typename... T2>
std::ranges::cartesian_product_view<T1, T2...> f2(T1 a0, T2... a1)
  requires((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return std::ranges::cartesian_product_view<T1, T2...>(a0, a1...);
}

template <typename T1, typename... T2>
auto f3(std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires((!std::ranges::__detail::__simple_view<T1> || ... ||
            !std::ranges::__detail::__simple_view<T2>)) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.begin();
}

template <typename T1, typename... T2>
auto f4(const std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires((std::ranges::range<const T1> && ... &&
            std::ranges::range<const T2>)) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.begin();
}

template <typename T1, typename... T2>
auto f5(std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires(((!std::ranges::__detail::__simple_view<T1> || ... ||
             !std::ranges::__detail::__simple_view<T2>) &&
            std::ranges::__detail::__cartesian_product_is_common<T1, T2...>)) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.end();
}

template <typename T1, typename... T2>
auto f6(const std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires(std::ranges::__detail::__cartesian_product_is_common<const T1,
                                                                const T2...>) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.end();
}

template <typename T1, typename... T2>
std::default_sentinel_t
f7(const std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.end();
}

template <typename T1, typename... T2>
auto f8(std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires(std::ranges::__detail::__cartesian_product_is_sized<T1, T2...>) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.size();
}

template <typename T1, typename... T2>
auto f9(const std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires(std::ranges::__detail::__cartesian_product_is_sized<const T1,
                                                               const T2...>) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.size();
}

template <typename T1, typename... T2>
bool f10(std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires(std::ranges::forward_range<
               std::ranges::cartesian_product_view<T1, T2...>> &&
           (!std::ranges::sized_range<
               std::ranges::cartesian_product_view<T1, T2...>>)) &&
          (std::is_class_v<std::ranges::cartesian_product_view<T1, T2...>> &&
           std::same_as<std::ranges::cartesian_product_view<T1, T2...>,
                        std::remove_cv_t<
                            std::ranges::cartesian_product_view<T1, T2...>>>) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.empty();
}

template <typename T1, typename... T2>
bool f11(std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires(std::ranges::sized_range<
              std::ranges::cartesian_product_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::cartesian_product_view<T1, T2...>> &&
           std::same_as<std::ranges::cartesian_product_view<T1, T2...>,
                        std::remove_cv_t<
                            std::ranges::cartesian_product_view<T1, T2...>>>) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.empty();
}

template <typename T1, typename... T2>
bool f12(const std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires(std::ranges::forward_range<
               const std::ranges::cartesian_product_view<T1, T2...>> &&
           (!std::ranges::sized_range<
               const std::ranges::cartesian_product_view<T1, T2...>>)) &&
          (std::is_class_v<std::ranges::cartesian_product_view<T1, T2...>> &&
           std::same_as<std::ranges::cartesian_product_view<T1, T2...>,
                        std::remove_cv_t<
                            std::ranges::cartesian_product_view<T1, T2...>>>) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.empty();
}

template <typename T1, typename... T2>
bool f13(const std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires(std::ranges::sized_range<
              const std::ranges::cartesian_product_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::cartesian_product_view<T1, T2...>> &&
           std::same_as<std::ranges::cartesian_product_view<T1, T2...>,
                        std::remove_cv_t<
                            std::ranges::cartesian_product_view<T1, T2...>>>) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.empty();
}

template <typename T1, typename... T2>
bool f14(std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return static_cast<bool>(a0);
}

template <typename T1, typename... T2>
bool f15(const std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return static_cast<bool>(a0);
}

template <typename T1, typename... T2>
auto f16(std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires(std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::cartesian_product_view<T1, T2...>>>) &&
          (std::is_class_v<std::ranges::cartesian_product_view<T1, T2...>> &&
           std::same_as<std::ranges::cartesian_product_view<T1, T2...>,
                        std::remove_cv_t<
                            std::ranges::cartesian_product_view<T1, T2...>>>) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.data();
}

template <typename T1, typename... T2>
auto f17(const std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires(std::ranges::range<
               const std::ranges::cartesian_product_view<T1, T2...>> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               const std::ranges::cartesian_product_view<T1, T2...>>>) &&
          (std::is_class_v<std::ranges::cartesian_product_view<T1, T2...>> &&
           std::same_as<std::ranges::cartesian_product_view<T1, T2...>,
                        std::remove_cv_t<
                            std::ranges::cartesian_product_view<T1, T2...>>>) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.data();
}

template <typename T1, typename... T2>
decltype(auto) f18(std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires(std::ranges::forward_range<
              std::ranges::cartesian_product_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::cartesian_product_view<T1, T2...>> &&
           std::same_as<std::ranges::cartesian_product_view<T1, T2...>,
                        std::remove_cv_t<
                            std::ranges::cartesian_product_view<T1, T2...>>>) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.front();
}

template <typename T1, typename... T2>
decltype(auto) f19(const std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires(std::ranges::forward_range<
              const std::ranges::cartesian_product_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::cartesian_product_view<T1, T2...>> &&
           std::same_as<std::ranges::cartesian_product_view<T1, T2...>,
                        std::remove_cv_t<
                            std::ranges::cartesian_product_view<T1, T2...>>>) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.front();
}

template <typename T1, typename... T2>
decltype(auto) f20(std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::cartesian_product_view<T1, T2...>> &&
           std::ranges::common_range<
               std::ranges::cartesian_product_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::cartesian_product_view<T1, T2...>> &&
           std::same_as<std::ranges::cartesian_product_view<T1, T2...>,
                        std::remove_cv_t<
                            std::ranges::cartesian_product_view<T1, T2...>>>) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.back();
}

template <typename T1, typename... T2>
decltype(auto) f21(const std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::cartesian_product_view<T1, T2...>> &&
           std::ranges::common_range<
               const std::ranges::cartesian_product_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::cartesian_product_view<T1, T2...>> &&
           std::same_as<std::ranges::cartesian_product_view<T1, T2...>,
                        std::remove_cv_t<
                            std::ranges::cartesian_product_view<T1, T2...>>>) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.back();
}

template <typename T1, typename... T2>
auto f22(std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires(std::ranges::input_range<
              std::ranges::cartesian_product_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::cartesian_product_view<T1, T2...>> &&
           std::same_as<std::ranges::cartesian_product_view<T1, T2...>,
                        std::remove_cv_t<
                            std::ranges::cartesian_product_view<T1, T2...>>>) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.cbegin();
}

template <typename T1, typename... T2>
auto f23(const std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires(std::ranges::input_range<
              const std::ranges::cartesian_product_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::cartesian_product_view<T1, T2...>> &&
           std::same_as<std::ranges::cartesian_product_view<T1, T2...>,
                        std::remove_cv_t<
                            std::ranges::cartesian_product_view<T1, T2...>>>) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.cbegin();
}

template <typename T1, typename... T2>
auto f24(std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires(std::ranges::input_range<
              std::ranges::cartesian_product_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::cartesian_product_view<T1, T2...>> &&
           std::same_as<std::ranges::cartesian_product_view<T1, T2...>,
                        std::remove_cv_t<
                            std::ranges::cartesian_product_view<T1, T2...>>>) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.cend();
}

template <typename T1, typename... T2>
auto f25(const std::ranges::cartesian_product_view<T1, T2...> &a0)
  requires(std::ranges::input_range<
              const std::ranges::cartesian_product_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::cartesian_product_view<T1, T2...>> &&
           std::same_as<std::ranges::cartesian_product_view<T1, T2...>,
                        std::remove_cv_t<
                            std::ranges::cartesian_product_view<T1, T2...>>>) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.cend();
}

template <typename T1, typename... T2, typename T3>
decltype(auto) f26(std::ranges::cartesian_product_view<T1, T2...> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::cartesian_product_view<T1, T2...>> &&
           std::same_as<std::ranges::cartesian_product_view<T1, T2...>,
                        std::remove_cv_t<
                            std::ranges::cartesian_product_view<T1, T2...>>>) &&
          (std::ranges::random_access_range<T3>) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.template operator[]<T3>(a1);
}

template <typename T1, typename... T2, typename T3>
decltype(auto) f27(const std::ranges::cartesian_product_view<T1, T2...> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::cartesian_product_view<T1, T2...>> &&
           std::same_as<std::ranges::cartesian_product_view<T1, T2...>,
                        std::remove_cv_t<
                            std::ranges::cartesian_product_view<T1, T2...>>>) &&
          (std::ranges::random_access_range<T3>) &&
          ((std::ranges::view<T1> && ... && std::ranges::view<T2>))
{
  return a0.template operator[]<T3>(a1);
}
